#include <deque>
#include <iostream>
#include <fstream>
#include <map>
#include <sstream>
#include <set>
#include <vector>

template <typename T> int sgn(T val)
{
    return (T(0) < val) - (val < T(0));
}

struct Coord {
	int x;
	int y;
};

bool operator==(const Coord &a, const Coord &b)
{
	return a.x == b.x && a.y == b.y;
}

bool operator!=(const Coord &a, const Coord &b)
{
	return !(a == b);
}

bool operator<(const Coord &a, const Coord &b)
{
	if (a.y != b.y) return a.y < b.y;
	return a.x < b.x;
}

Coord operator+(const Coord &a, const Coord &b)
{
	return {a.x + b.x, a.y + b.y};
}

Coord &operator+=(Coord &a, const Coord &b)
{
	a.x += b.x;
	a.y += b.y;
	return a;
}

std::ostream &operator<<(std::ostream &os, const Coord &coord)
{
	os << coord.x << ',' << coord.y;
	return os;
}

std::vector<Coord> split_coords(const std::string &s, std::string_view delim)
{
	std::vector<Coord> res;

	size_t last = 0;
	size_t next = 0;
	while ((next = s.find(delim, last)) != std::string::npos) {
		auto sub = s.substr(last, next-last);
		size_t comma = sub.find(',');
		res.push_back({std::stoi(sub.substr(0, comma)), std::stoi(sub.substr(comma + 1))});
		last = next + delim.size();
	}
	{
		auto sub = s.substr(last);
		size_t comma = sub.find(',');
		res.push_back({std::stoi(sub.substr(0, comma)), std::stoi(sub.substr(comma + 1))});
	}
	return res;
}

constexpr Coord SAND_START{500, 0};

Coord pour_sand_into_abyss(const std::set<Coord> &rocks)
{
	int max_y = rocks.rbegin()->y; // because of the ordering, the maximum y is at the end of the set

	Coord sand = SAND_START;
	// move
	while (sand.y < max_y) {
		if (rocks.find(sand + Coord{0, 1}) == rocks.end()) {
			sand += {0, 1}; // move down
		} else if (rocks.find(sand + Coord{-1, 1}) == rocks.end()) {
			sand += {-1, 1}; // move down & left
		} else if (rocks.find(sand + Coord{1, 1}) == rocks.end()) {
			sand += {1, 1}; // move down & right
		} else {
			return sand;
		}
	}
	return {0, 0};
}

// flood fill all possible positions that sand can reach
size_t pour_sand_onto_floor(const std::set<Coord> &rocks, int floor)
{
	std::set<Coord> visited;
	std::deque<Coord> to_visit;
	to_visit.push_back(SAND_START);

	while (!to_visit.empty()) {
		auto next = to_visit.front();
		to_visit.pop_front();
		if (visited.find(next) != visited.end()) continue; // have we already visited this via another neighbour?
		visited.insert(next);

		auto coords = {next + Coord{0, 1}, next + Coord{-1, 1}, next + Coord{1, 1}}; // valid neighbours
		for (const auto &c : coords) {
			if (rocks.find(c) == rocks.end() && visited.find(c) == visited.end() && c.y < floor) {
				to_visit.push_back(c);
			}
		}
	}
	return visited.size();
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

	std::string example_input = "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9";
	std::stringstream ex_input(example_input);

	std::vector<std::vector<Coord>> points;

	for (std::string line; std::getline(input, line); ) {
		points.emplace_back(split_coords(line, " -> "));
	}

	std::set<Coord> rocks;

	for (const auto &point_list : points) {
		for (size_t i = 0; i < point_list.size() - 1; i++) {
			const Coord start = point_list[i];
			const Coord end = point_list[i + 1];
			const Coord direction = {sgn(end.x - start.x), sgn(end.y - start.y)};
			for (Coord pointer = start; pointer != end; pointer += direction) {
				rocks.insert(pointer);
			}
			rocks.insert(end);
		}
	}

	auto abyss_rocks = rocks;

	for (Coord pour_result = pour_sand_into_abyss(abyss_rocks); pour_result != Coord{0, 0}; pour_result = pour_sand_into_abyss(abyss_rocks)) {
		abyss_rocks.insert(pour_result);
	}

	int max_y = rocks.rbegin()->y; // because of the ordering, the maximum y is at the end of the set
	int floor = max_y + 2;
	std::cout << "Number of sand particles able to come to rest: " << abyss_rocks.size() - rocks.size() << '\n';
	std::cout << "Number of sand particles able to come to rest with a floor: " << pour_sand_onto_floor(rocks, floor) << '\n';
}
