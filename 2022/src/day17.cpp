#include <algorithm>
#include <array>
#include <fstream>
#include <iostream>
#include <iterator>
#include <numeric>
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
	int max_y = std::max(cur_rock_pos.y, stopped_rocks.rbegin()->y);
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

	//jet_pattern = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

	std::set<Coord> stopped_rocks;
	stopped_rocks.insert(Coord{-1, -1}); // just so it's non-empty
	size_t jet_idx = 0;
	Rock cur_rock = rocks[0];
	Coord rock_pos{-1, -1};
	int rock_count = 0;

	std::vector<int> heights;

	while (rock_count < 10'000) {
		// spawn new rock if needed
		if (rock_pos == Coord{-1, -1}) {
			// ordering of set allows us to just take the last element
			int max_y = stopped_rocks.rbegin()->y; // -1 when empty means we get +3 + rock height properly
			heights.push_back(max_y); // record height after N stopped rocks

			cur_rock = rocks[rock_count % rocks.size()];

			rock_count++;
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

	const size_t P1_ROCK_COUNT = 2022;
	const int64_t P2_ROCK_COUNT = 1'000'000'000'000;

	// Go looking for a cycle
	int cycle_length = 0;
	int cycle_start = 0;
	// cycle length must be a multiple of 5 - the number of rocks that are available
	for (int run_length = rocks.size(); cycle_length == 0 && run_length < (int)heights.size() / 5; run_length += rocks.size()) {
		// assumes cycle establishes itself within 1/5 of the total heights we've generated
		// also allows us to more thoroughly check that we've got a cycle (i.e. at least 4 repeats)
		for (auto it = heights.begin(); it != heights.begin() + (heights.size() / 5); ++it) {
			// get the initial difference
			const std::vector<int> initial_vec(it, it + run_length - 1);
			const std::vector<int> next_vec(it + run_length, it + 2 * run_length - 1);
			std::vector<int> initial_diffs;
			std::transform(next_vec.begin(), next_vec.end(), initial_vec.begin(), std::back_inserter(initial_diffs), std::minus<>());

			bool all_matches = true;
			// looks for sub vectors with matching differences between elements
			// could probably do this without the copies if you tried hard enough
			for (auto jt = it + run_length; jt < heights.end() - run_length - 1; jt += run_length) {
				const std::vector<int> base_vec(jt, jt + run_length - 1);
				const std::vector<int> compare_vec(jt + run_length, jt + 2 * run_length - 1);
				std::vector<int> diffs;
				std::transform(compare_vec.begin(), compare_vec.end(), base_vec.begin(), std::back_inserter(diffs), std::minus<>());
				if (diffs != initial_diffs) {
					all_matches = false;
					break;
				}
			}
			if (all_matches) {
//				std::cout << "Found a cycle?!\n";
//				std::cout << "Starts at rock count: " << std::distance(heights.begin(), it) << " with a height value of " << *it << '\n';
//				std::cout << "Cycle repeats every " << run_length << " rocks, with a change in height of " << *(it + run_length) - *it << '\n';
//				for (auto jt = it; jt < heights.end(); jt += run_length) {
//					std::cout << std::distance(heights.begin(), jt) << '(' << *jt << ") ";
//				}
//				std::cout << '\n';
				cycle_start = std::distance(heights.begin(), it);
				cycle_length = run_length;
				break;
			}
		}
	}

	if (cycle_length == 0) {
		// shuts the linter up about a potential divide by zero
		throw "Could not find cycle after lots of iterations";
	}

	std::cout << "Maximum height after 2022 rocks: " << heights[P1_ROCK_COUNT] + 1 << '\n'; // +1 because units, not index

	int64_t remaining_rocks = P2_ROCK_COUNT - (cycle_start + cycle_length);
	int64_t multiplier = remaining_rocks / cycle_length;
	int64_t offset = remaining_rocks % cycle_length;

	int height_diff = heights[cycle_start + cycle_length] - heights[cycle_start];
	int64_t total_height = heights[cycle_start + cycle_length];
	total_height += height_diff * multiplier;
	total_height += heights[cycle_start + offset] - heights[cycle_start];
	std::cout << "Maximum height after 1 trillion rocks: " << total_height + 1 << '\n';
}
