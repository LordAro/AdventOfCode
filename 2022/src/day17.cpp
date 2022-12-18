#include <algorithm>
#include <array>
#include <fstream>
#include <iostream>
#include <set>
#include <vector>

using Rock = std::vector<std::vector<bool>>;

const std::array<Rock, 5> rocks{
	Rock{
		{ true, true, true, true }, // ####
	},
	Rock{
		{ false, true, false }, // .#.
		{ true, true, true },   // ###
		{ false, true, false }, // .#.
	},
	Rock{
		{ false, false, true }, // ..#
		{ false, false, true }, // ..#
		{ true, true, true },   // ###
	},
	Rock{
		{ true }, // #
		{ true }, // #
		{ true }, // #
		{ true }, // #
	},
	Rock{
		{ true, true }, // ##
		{ true, true }, // ##
	}
};

const int GRID_WIDTH = 7;

struct Coord {
	int x, y;

	bool operator==(const Coord &other) const
	{
		return this->x == other.x && this->y == other.y;
	}

	bool operator<(const Coord &other) const
	{
		if (this->y != other.y) return this->y < other.y;
		return this->x < other.x;
	}
};

std::vector<Coord> get_rock_coords(const Coord &cur_pos, const Rock &rock)
{
	std::vector<Coord> coords;
	for (int y = 0; y < (int)rock.size(); y++) {
		for (int x = 0; x < (int)rock[y].size(); x++) {
			if (!rock[y][x]) continue;

			int new_x = cur_pos.x + x;
			int new_y = cur_pos.y - y; // stopped_rocks are on a grid that is +x, +y, whereas rock is +x,-y
			coords.push_back({new_x, new_y});
		}
	}
	return coords;
}

// debugging
void print(const std::set<Coord> &stopped_rocks, const Rock &cur_rock, const Coord &cur_rock_pos)
{
	const std::vector<Coord> rock_coords = get_rock_coords(cur_rock_pos, cur_rock);
	int max_y = std::max(cur_rock_pos.y, stopped_rocks.empty() ? 0 : stopped_rocks.rbegin()->y);
	for (int y = max_y; y >= 0; y--) {
		std::cout << '|';
		for (int x = 0; x < GRID_WIDTH; x++) {
			Coord c{x, y};
			if (std::find(rock_coords.begin(), rock_coords.end(), c) != rock_coords.end()) {
				std::cout << '@';
			} else if (stopped_rocks.find(c) != stopped_rocks.end()) {
				std::cout << '#';
			} else {
				std::cout << ' ';
			}
		}
		std::cout << "|\n";
	}
	std::cout << "+-------+\n";
}

bool try_move(const std::set<Coord> &stopped_rocks, const Coord &cur_pos, const Rock &rock, int dir)
{
	Coord new_pos = cur_pos;
	switch (dir) {
		case 0: // down
			new_pos.y--;
			break;
		case 1: // left
			new_pos.x--;
			break;
		case 2: // right
			new_pos.x++;
			break;
		default:
			__builtin_unreachable();
	}

	for (const auto &rock_coord : get_rock_coords(new_pos, rock)) {
		if (rock_coord.x < 0 || rock_coord.x >= GRID_WIDTH) {
			return false; // chamber is 7 wide
		}
		if (rock_coord.y < 0) {
			return false; // stop at the floor (but rocks are allowed to "slide" along the floor)
		}
		if (stopped_rocks.find(rock_coord) != stopped_rocks.end()) {
			return false;
		}
	}
	return true;
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

	std::string jet_pattern;
	std::getline(input, jet_pattern);

//	jet_pattern = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

	std::set<Coord> stopped_rocks;
	size_t jet_idx = 0;
	size_t next_rock_idx = 0;
	Rock cur_rock = rocks[0];
	Coord rock_pos{-1, -1};
	int rock_count = 0;
	while (true) {
		// spawn new rock if needed
		if (rock_pos == Coord{-1, -1}) {
			rock_count++;
			if (rock_count > 2022) break; // done.

			// ordering of set allows us to just take the last element
			int max_y = !stopped_rocks.empty() ? stopped_rocks.rbegin()->y : -1;
			cur_rock = rocks[next_rock_idx];
			next_rock_idx = (next_rock_idx + 1) % rocks.size();

			rock_pos = {2, max_y + 3 + (int)cur_rock.size()}; //starting point of top left
		}

		// try to move left/right
		int next_movement = jet_pattern[jet_idx] == '<' ? 1 : 2;
		jet_idx = (jet_idx + 1) % jet_pattern.size();
		if (try_move(stopped_rocks, rock_pos, cur_rock, next_movement)) {
			rock_pos.x = rock_pos.x + (next_movement == 1 ? -1 : 1);
		} else {
			// nothing happens if can't move left/right
		}

		// try to move down
		if (try_move(stopped_rocks, rock_pos, cur_rock, 0)) {
			rock_pos.y--;
		} else {
			// convert rock to stopped_rocks
			const auto rock_coords = get_rock_coords(rock_pos, cur_rock);
			stopped_rocks.insert(rock_coords.begin(), rock_coords.end());
			rock_pos = {-1, -1};
		}

	}
//			print(stopped_rocks, cur_rock, rock_pos);
//			std::cout << "\n\n";

	std::cout << "Maximum height after 2022 rocks: " << stopped_rocks.rbegin()->y + 1 << '\n';
}
