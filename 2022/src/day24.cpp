#include <algorithm>
#include <array>
#include <fstream>
#include <iostream>
#include <queue>
#include <set>
#include <sstream>
#include <vector>

struct Coord {
	int x, y;

	bool operator<(const Coord &other) const
	{
		if (this->y != other.y) return this->y < other.y;
		return this->x < other.x;
	}

	bool operator>(const Coord &other) const
	{
		if (this->y != other.y) return this->y > other.y;
		return this->x > other.x;
	}

	bool operator==(const Coord &other) const
	{
		return this->y == other.y && this->x == other.x;
	}

	bool operator!=(const Coord &other) const
	{
		return !(*this == other);
	}

	Coord operator+(const Coord &other) const
	{
		return {this->x + other.x, this->y + other.y};
	}

	friend std::ostream &operator<<(std::ostream &os, const Coord &c)
	{
		return (os << c.x << ',' << c.y);
	}
};

enum class Dir {
	North,
	East,
	South,
	West,
};

const std::array<Coord, 4> change_coord{{ Coord{0, -1}, Coord{1, 0}, Coord{0, 1}, Coord{-1, 0} }};

using BlizzardPositions = std::set<std::pair<Coord, Dir>>;

std::vector<BlizzardPositions> _cached_positions;
BlizzardPositions get_blizzards_at(const std::set<Coord> &walls, int time)
{
	if (time < (int)_cached_positions.size()) {
		return _cached_positions[time];
	}
	const BlizzardPositions &previous_positions = get_blizzards_at(walls, time - 1); // recurse!
	BlizzardPositions positions;

	const auto &[min, max] = std::minmax_element(walls.begin(), walls.end());
	for (const auto &blizzard : previous_positions) {
		// helpfully, it seems we don't need to worry about blizzards not intersecting with walls
		const auto &[pos, dir] = blizzard;
		Coord new_pos = pos + change_coord[(int)dir];
		if (walls.find(new_pos) != walls.end()) {
			// intersected a wall, restart at the other side
			Coord wall_pos{};
			switch (dir) {
				case Dir::North:
					// find wall with maximum y (matching x)
					wall_pos = {pos.x, max->y};
					break;
				case Dir::East:
					// find wall with minimum x (matching y)
					wall_pos = {min->x, pos.y};
					break;
				case Dir::South:
					// find wall with minimum y (matching x)
					wall_pos = {pos.x, min->y};
					break;
				case Dir::West:
					// find wall with maximum x (matching y)
					wall_pos = {max->x, pos.y};
					break;
			}
			new_pos = wall_pos + change_coord[(int)dir]; // get us off the wall
		}
		positions.emplace(new_pos, dir);
	}
	_cached_positions.push_back(positions);
	return positions;
}

int manhattan_distance(const Coord &a, const Coord &b)
{
	return std::abs(a.x - b.x) + std::abs(a.y - b.y);
}

int get_route_time(const std::set<Coord> &walls, const Coord start, const Coord end, int start_time)
{
	// bounding box of walls - assumes a rectangular box (with start/end within it)
	const auto &[min, max] = std::minmax_element(walls.begin(), walls.end());
	using CoordTime = std::pair<Coord, int>;
	std::set<CoordTime> visited;

	// using "greater" comparison makes it a "min priority queue"
	const auto order_by_closest = [end](const CoordTime &a, const CoordTime &b) {
		// shortest time is more important than shortest distance
		int man_a = manhattan_distance(a.first, end) + a.second;
		int man_b = manhattan_distance(b.first, end) + b.second;
		if (man_a != man_b) return man_a > man_b;
		return a.first > b.first;
	};

	std::priority_queue<CoordTime, std::vector<CoordTime>, decltype(order_by_closest)> to_search(order_by_closest);
	to_search.emplace(start, start_time);

	while (!to_search.empty()) {
		const auto pos_time = to_search.top();
		to_search.pop();
		if (visited.find(pos_time) != visited.end()) continue;
		visited.insert(pos_time);
		const auto &[pos, time] = pos_time;
//		std::cout << pos << ' ' << time << '\n';

		if (pos == end) {
			return time;
		}

		// add free neighbours to search list
		const auto &next_blizzard_positions = get_blizzards_at(walls, time + 1);
		Coord new_pos = pos;
		if (std::find_if(next_blizzard_positions.begin(), next_blizzard_positions.end(),
					[new_pos](const auto &pos_dir) { return pos_dir.first == new_pos; })
				== next_blizzard_positions.end()) {
//			std::cout << "Adding: " << new_pos << ' ' << time + 1 << '\n';
			to_search.emplace(new_pos, time + 1); // not moving, don't need to check edges
		}
		for (int i = 0; i < 4; i++) {
			new_pos = pos + change_coord[i];
			if (std::find_if(next_blizzard_positions.begin(), next_blizzard_positions.end(),
						[new_pos](const auto &pos_dir) { return pos_dir.first == new_pos; })
					!= next_blizzard_positions.end()) {
				continue;
			}

			// also make sure we don't go out of bounds
			if (new_pos != start && new_pos != end && (new_pos.x <= min->x || new_pos.x >= max->x || new_pos.y <= min->y || new_pos.y >= max->y)) continue;
//			std::cout << "Adding: " << new_pos << ' ' << time + 1 << '\n';
			to_search.emplace(new_pos, time + 1);
		}
	}
	return -1;
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

	std::stringstream ex_input(
			"#.######\n"
"#>>.<^<#\n"
"#.<..<<#\n"
"#>v.><>#\n"
"#<^v^^>#\n"
"######.#");

	std::set<Coord> walls;
	BlizzardPositions blizzards;

	Coord start_pos{-1, -1};
	Coord end_pos{-1, -1};
	int y = 0;
	for (std::string line; std::getline(input, line); y++) {
		for (int x = 0; x < (int)line.size(); x++) {
			Coord c{x, y};
			switch (line[x]) {
				case '.':
					// first '.' is our start
					// last '.' is our dest
					if (start_pos.x == -1) {
						start_pos = c;
					}
					end_pos = c;
					break;
				case '<':
					blizzards.emplace(c, Dir::West);
					break;
				case '>':
					blizzards.emplace(c, Dir::East);
					break;
				case '^':
					blizzards.emplace(c, Dir::North);
					break;
				case 'v':
					blizzards.emplace(c, Dir::South);
					break;
				case '#':
					walls.insert(c);
					break;
				default:
					__builtin_unreachable();
			}
		}
	}

	_cached_positions.push_back(blizzards); // time 0

	int time = get_route_time(walls, start_pos, end_pos, 0);
	std::cout << "Time taken to cross blizzard: " << time << '\n';

	int time2 = get_route_time(walls, end_pos, start_pos, time); // back to start
	int time3 = get_route_time(walls, start_pos, end_pos, time2); // and back to finish
	std::cout << "Time taken to cross blizzard 3 times: " << time3 << '\n';
}
