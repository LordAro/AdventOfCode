#include <algorithm>
#include <deque>
#include <fstream>
#include <iostream>
#include <sstream>
#include <set>
#include <map>
#include <vector>

// To save passing strings around everywhere.
class ValveName {
	int i;
public:
	ValveName() = default;
	ValveName(const std::string_view s) : i((s[0] << 8) + s[1]) {}

	bool operator==(const ValveName &other) const
	{
		return this->i == other.i;
	}

	bool operator!=(const ValveName &other) const
	{
		return this->i != other.i;
	}

	bool operator<(const ValveName &other) const
	{
		return this->i < other.i;
	}

	std::string str() const
	{
		char c1 = i >> 8;
		char c2 = i & 0xf;
		return std::string{c1, c2};
	}
};

struct Valve {
	ValveName name;
	int flow_rate;
	std::vector<ValveName> links;

	int total_flow_at_time(int max, int t) const
	{
		return this->flow_rate * (max - t);
	}

	bool operator==(const Valve &other) const
	{
		return this->name == other.name;
	}
};

// debugging
std::ostream &operator<<(std::ostream &os, const Valve &v)
{
	os << v.name.str() << " (" << v.flow_rate << ") - ";
	for (const auto &l : v.links) os << l.str() << ", ";
	return os;
}

std::vector<ValveName> split_by(const std::string &s, std::string_view delim)
{
	std::vector<ValveName> res;

	size_t last = 0;
	size_t next = 0;
	while ((next = s.find(delim, last)) != std::string::npos) {
		res.emplace_back(s.substr(last, next - last));
		last = next + delim.size();
	}
	{
		res.emplace_back(s.substr(last));
	}
	return res;
}

std::istream &operator>>(std::istream &is, Valve &valve)
{
	std::string s;
	is >> s >> s; // Valve <name>
	valve.name = ValveName(s);
	is >> s >> s; // has flow
	is.ignore(6); // ' rate='
	is >> valve.flow_rate;
	is.ignore(2); // '; '
	is >> s >> s >> s >> s; // tunnels lead to valve(s)
	is.ignore(1); // ' '
	std::string links;
	std::getline(is, links);
	valve.links = split_by(links, ", ");
	return is;
}

int get_path_cost(const std::map<ValveName, Valve> &valves, const ValveName &source, const ValveName &dest)
{
	std::set<ValveName> visited;
	std::deque<std::pair<ValveName, int>> to_visit;
	to_visit.push_back({source, 0});

	while (!to_visit.empty()) {
		const auto [next, depth] = to_visit.front();
		to_visit.pop_front();

		if (next == dest) return depth;
		if (visited.find(next) != visited.end()) continue;

		for (const auto &l : valves.at(next).links) {
			to_visit.push_back({l, depth + 1});
		}
	}
	// all nodes are reachable
	__builtin_unreachable();
}

int dfs(const std::map<ValveName, Valve> &valves, const std::map<std::pair<ValveName, ValveName>, int> &route_costs, const std::vector<std::pair<ValveName, int>> &route)
{
	const int TOTAL_TIME = 30;
	const auto [cur_valve, cur_time] = route.back();
	int max_route_cost = 0;
	for (const auto &[n, valve] : valves) {
		if (valve.flow_rate == 0) continue;
		if (std::find_if(route.begin(), route.end(), [v_name=valve.name](const std::pair<ValveName, int> &p) { return p.first == v_name; }) != route.end()) continue;
		int remaining_time = TOTAL_TIME - cur_time;
		int route_cost = route_costs.at({cur_valve, valve.name});
		if (route_cost > remaining_time) continue;

		auto new_route = route;
		new_route.emplace_back(valve.name, cur_time + route_cost + 1); // costs 1 min to open valve
		max_route_cost = std::max(max_route_cost, dfs(valves, route_costs, new_route));
	}

	// no more possible routes
	if (max_route_cost == 0) {
		int flow_total = 0;
		for (const auto &[v, n] : route) {
			flow_total += valves.at(v).total_flow_at_time(TOTAL_TIME, n);
		}
		return flow_total;
	}
	return max_route_cost;
}

std::vector<Valve> get_ordered_possible_moves(const std::map<std::pair<ValveName, ValveName>, int> &route_costs, const int total_time, std::vector<Valve> valves, ValveName cur_pos, int cur_time)
{
	for (auto it = valves.begin(); it != valves.end(); ) {
		int route_cost = route_costs.at({cur_pos, it->name});
		if (route_cost > (total_time - cur_time)) {
			it = valves.erase(it);
		} else {
			++it;
		}
	}

	// sort valves by maximum possible flow rate
	std::sort(valves.rbegin(), valves.rend(), [&](const Valve &a, const Valve &b){
		int route_cost_a = route_costs.at({cur_pos, a.name});
		int route_cost_b = route_costs.at({cur_pos, b.name});

		int flow_total_a = a.total_flow_at_time(total_time, cur_time - route_cost_a - 1);
		int flow_total_b = b.total_flow_at_time(total_time, cur_time - route_cost_b - 1);
		return flow_total_a < flow_total_b;
	});
	return valves;
}

using RouteItem = std::pair<ValveName, int>; // location, time
using Route = std::vector<RouteItem>;

int dfs_with_elephant(const std::map<ValveName, Valve> &valves, const std::map<std::pair<ValveName, ValveName>, int> &route_costs, const std::pair<Route, Route> &routes)
{
	const int TOTAL_TIME = 26;

	std::vector<Valve> unused_valves;
	for (const auto &[_, valve] : valves) {
		if (valve.flow_rate == 0) continue;
		if (std::find_if(routes.first.begin(), routes.first.end(),
				[v_name=valve.name](const std::pair<ValveName, int> &p) { return p.first == v_name; }) != routes.first.end()
		) {
			continue;
		}
		if (std::find_if(routes.second.begin(), routes.second.end(),
				[v_name=valve.name](const std::pair<ValveName, int> &p) { return p.first == v_name; }) != routes.second.end()
		) {
			continue;
		}
		unused_valves.push_back(valve);
	}

	const auto [cur_valve, cur_time] = routes.first.back();
	const auto [ele_valve, ele_time] = routes.second.back();

	const auto possible_moves = get_ordered_possible_moves(route_costs, TOTAL_TIME, unused_valves, cur_valve, cur_time);
	const auto possible_ele_moves = get_ordered_possible_moves(route_costs, TOTAL_TIME, unused_valves, ele_valve, ele_time);

	int max_route_cost = 0;

	if (possible_moves.empty() && possible_ele_moves.empty()) {
		// no more possible routes
		int flow_total = 0;
		for (const auto &[v, n] : routes.first) {
			flow_total += valves.at(v).total_flow_at_time(TOTAL_TIME, n);
		}
		for (const auto &[v, n] : routes.second) {
			flow_total += valves.at(v).total_flow_at_time(TOTAL_TIME, n);
		}
		return flow_total;
	} else if (possible_moves.empty() && !possible_ele_moves.empty()) {
		for (const auto &next_valve : possible_ele_moves) {
			auto new_routes = routes;
			int route_cost = route_costs.at({ele_valve, next_valve.name});
			new_routes.second.emplace_back(next_valve.name, ele_time + route_cost + 1); // costs 1 min to open valve
			max_route_cost = std::max(max_route_cost, dfs_with_elephant(valves, route_costs, new_routes));
		}
	} else if (possible_ele_moves.empty() && !possible_moves.empty()) {
		for (const auto &next_valve : possible_moves) {
			auto new_routes = routes;
			int route_cost = route_costs.at({cur_valve, next_valve.name});
			new_routes.first.emplace_back(next_valve.name, cur_time + route_cost + 1); // costs 1 min to open valve
			max_route_cost = std::max(max_route_cost, dfs_with_elephant(valves, route_costs, new_routes));
		}
	} else {
		for (const auto &next_valve : possible_moves) {
			for (const auto &next_ele_valve : possible_ele_moves) {
				if (next_valve == next_ele_valve) {
					continue; // TODO: something else...?
				}

				int route_cost = route_costs.at({cur_valve, next_valve.name});
				int route_cost_ele = route_costs.at({ele_valve, next_ele_valve.name});

				auto new_routes = routes;
				new_routes.first.emplace_back(next_valve.name, cur_time + route_cost + 1); // costs 1 min to open valve
				new_routes.second.emplace_back(next_ele_valve.name, ele_time + route_cost_ele + 1); // costs 1 min to open valve
				max_route_cost = std::max(max_route_cost, dfs_with_elephant(valves, route_costs, new_routes));
			}
		}
	}
	return max_route_cost;
}

int main(int argc, char **argv)
{
	if (argc != 2) {
		std::cerr << "Incorrect number of arguments provided\n";
		return 1;
	}
	std::fstream input(argv[1]);
	if (!input) {
		std::cerr << "Could not open input file\n";
		return 1;
	}

	std::string example_input =
	"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB\n"
	"Valve BB has flow rate=13; tunnels lead to valves CC, AA\n"
	"Valve CC has flow rate=2; tunnels lead to valves DD, BB\n"
	"Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE\n"
	"Valve EE has flow rate=3; tunnels lead to valves FF, DD\n"
	"Valve FF has flow rate=0; tunnels lead to valves EE, GG\n"
	"Valve GG has flow rate=0; tunnels lead to valves FF, HH\n"
	"Valve HH has flow rate=22; tunnel leads to valve GG\n"
	"Valve II has flow rate=0; tunnels lead to valves AA, JJ\n"
	"Valve JJ has flow rate=21; tunnel leads to valve II\n";

	std::stringstream ex_input(example_input);


	std::map<ValveName, Valve> valves;
	for (Valve v; input >> v; ) {
		valves[v.name] = v;
	}

	std::map<std::pair<ValveName, ValveName>, int> route_costs;
	for (const auto &source : valves) {
		if (source.second.flow_rate == 0 && source.first != ValveName("AA")) continue;
		for (const auto &dest : valves) {
			if (source.first == dest.first || dest.second.flow_rate == 0) continue;
			route_costs[{source.first, dest.first}] = get_path_cost(valves, source.first, dest.first);
		}
	}

	std::vector<std::pair<ValveName, int>> route;
	route.emplace_back(ValveName("AA"), 0);
	std::cout << "Maximum flow rate: " << dfs(valves, route_costs, route) << '\n';

	std::pair<Route, Route> route_with_ele;
	route_with_ele.first.emplace_back(ValveName("AA"), 0);
	route_with_ele.second.emplace_back(ValveName("AA"), 0); // duplicate is fine, as AA doesn't have a flow
	std::cout << "Maximum flow rate with an elephant helping: " << dfs_with_elephant(valves, route_costs, route_with_ele) << '\n';
}
