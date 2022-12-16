#include <algorithm>
#include <deque>
#include <fstream>
#include <iostream>
#include <sstream>
#include <set>
#include <map>
#include <vector>

struct Valve {
	std::string name;
	int flow_rate;
	std::vector<std::string> links;

	int total_flow_at_time(int t) const
	{
		return this->flow_rate * (30 - t);
	}
};

// debugging
std::ostream &operator<<(std::ostream &os, const Valve &v)
{
	os << v.name << " (" << v.flow_rate << ") - ";
	for (const auto &l : v.links) os << l << ", ";
	return os;
}

std::vector<std::string> split_by(const std::string &s, std::string_view delim)
{
	std::vector<std::string> res;

	size_t last = 0;
	size_t next = 0;
	while ((next = s.find(delim, last)) != std::string::npos) {
		res.push_back(s.substr(last, next - last));
		last = next + delim.size();
	}
	{
		res.push_back(s.substr(last));
	}
	return res;
}

std::istream &operator>>(std::istream &is, Valve &valve)
{
	std::string s;
	is >> s; // Valve
	is >> valve.name;
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

int get_path_cost(const std::map<std::string, Valve> &valves, const std::string &source, const std::string &dest)
{
	std::set<std::string> visited;
	std::deque<std::pair<std::string, int>> to_visit;
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
// build map of cost from a valve to any (nonzero) valve
// build up list of (non-zero) valves
// sort by

int dfs(const std::map<std::string, Valve> &valves, const std::map<std::pair<std::string, std::string>, int> &route_costs, const std::vector<std::pair<std::string, int>> &route)
{
	const auto [cur_valve, cur_time] = route.back();
	int max_route_cost = 0;
	for (const auto &[n, valve] : valves) {
		if (valve.flow_rate == 0) continue;
		if (std::find_if(route.begin(), route.end(), [v_name=valve.name](const std::pair<std::string, int> &p) { return p.first == v_name; }) != route.end()) continue;
		int remaining_time = 30 - cur_time;
		int route_cost = route_costs.at({cur_valve, valve.name});
		if (route_cost > remaining_time) continue;

		auto new_route = route;
		new_route.push_back({valve.name, cur_time + route_cost + 1}); // costs 1 min to open valve
		max_route_cost = std::max(max_route_cost, dfs(valves, route_costs, new_route));
	}

	// no more possible routes
	if (max_route_cost == 0) {
		int flow_total = 0;
//		std::cout << "Route:\n";
		for (const auto &[v, n] : route) {
//			std::cout << v << " at " << n << " = " << valves.at(v).total_flow_at_time(n) << '\n';
			flow_total += valves.at(v).total_flow_at_time(n);
		}
		return flow_total;
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


	std::map<std::string, Valve> valves;
	for (Valve v; input >> v; ) {
		valves[v.name] = v;
	}

	std::map<std::pair<std::string, std::string>, int> route_costs;
	for (const auto &source : valves) {
		if (source.second.flow_rate == 0 && source.first != "AA") continue;
		for (const auto &dest : valves) {
			if (source.first == dest.first || dest.second.flow_rate == 0) continue;
			route_costs[{source.first, dest.first}] = get_path_cost(valves, source.first, dest.first);
		}
	}

	std::vector<std::pair<std::string, int>> route;
	route.push_back({"AA", 0});
	std::cout << "Maximum flow rate: " << dfs(valves, route_costs, route) << '\n';
}
