#include <algorithm>
#include <deque>
#include <fstream>
#include <iostream>
#include <set>
#include <vector>

struct Coord {
	int x;
	int y;
};

std::ostream &operator<<(std::ostream &output, const Coord &c)
{
	output << c.x << ',' << c.y;
	return output;
}

bool operator<(const Coord &a, const Coord &b)
{
	if (a.y != b.y) return a.y < b.y;
	return a.x < b.x;
}

bool operator==(const Coord &a, const Coord &b)
{
	return a.x == b.x && a.y == b.y;
}

using Route = std::vector<Coord>;
using Grid2D = std::vector<std::vector<int>>;

std::vector<Coord> possible_moves(const Grid2D &heightmap, Coord pos, bool reverse)
{
	auto check = !reverse
		? [](int newc, int curc) { return newc <= curc + 1; }
		: [](int newc, int curc) { return curc - 1 <= newc; };
	std::vector<Coord> moves;
	moves.reserve(4);
	int cur_height = heightmap[pos.y][pos.x];
	if (pos.y > 0) {
		int new_height = heightmap[pos.y - 1][pos.x];
		if (check(new_height, cur_height)) {
			moves.push_back({pos.x, pos.y - 1});
		}
	}
	if (pos.y < (int)heightmap.size() - 1) {
		int new_height = heightmap[pos.y + 1][pos.x];
		if (check(new_height, cur_height)) {
			moves.push_back({pos.x, pos.y + 1});
		}
	}
	if (pos.x > 0) {
		int new_height = heightmap[pos.y][pos.x - 1];
		if (check(new_height, cur_height)) {
			moves.push_back({pos.x - 1, pos.y});
		}
	}
	if (pos.x < (int)heightmap[pos.y].size() - 1) {
		int new_height = heightmap[pos.y][pos.x + 1];
		if (check(new_height, cur_height)) {
			moves.push_back({pos.x + 1, pos.y});
		}
	}
	return moves;
}

Route get_shortest_route(const Grid2D &heightmap, const Coord start_pos, const std::set<Coord> end_points, const bool reverse_check)
{
	std::set<Coord> visited;
	visited.insert(start_pos);

	std::deque<Route> queue;
	queue.push_back({start_pos});
	while (!queue.empty()) {
		auto partial_route = queue.front();
		queue.pop_front();

		const auto &pos = partial_route.back();
		if (end_points.find(pos) != end_points.end()) {
			return partial_route;
		}

		for (const auto &move : possible_moves(heightmap, pos, reverse_check)) {
			if (visited.find(move) != visited.end()) continue;
			Route r2 = partial_route;
			visited.insert(move);
			r2.push_back(move);
			queue.push_back(r2);
		}
	}
	return {}; // no route found :(
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

	Grid2D heightmap;
	std::set<Coord> lowest_points;
	Coord start_pos;
	Coord end_pos;

	std::string line;
	while(std::getline(input, line)) {
		std::vector<int> row;
		row.reserve(line.size());
		for (char c : line) {
			if (c == 'S') {
				start_pos = {(int)row.size(), (int)heightmap.size()};
				c = 'a';
			} else if (c == 'E') {
				end_pos = {(int)row.size(), (int)heightmap.size()};
				c = 'z';
			}
			if (c == 'a') {
				lowest_points.insert({(int)row.size(), (int)heightmap.size()});
			}
			row.push_back(c - 'a');
		}
		heightmap.push_back(row);
	}
	auto route = get_shortest_route(heightmap, start_pos, {end_pos}, false);
	std::cout << "Shortest route from starting point: " << route.size() - 1 << '\n'; // discount starting point

	auto route_to_lowest = get_shortest_route(heightmap, end_pos, lowest_points, true);
	std::cout << "Shortest route when starting from low points: " << route_to_lowest.size() - 1 << '\n';
}
