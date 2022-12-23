#include <algorithm>
#include <array>
#include <cassert>
#include <fstream>
#include <iostream>
#include <map>
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

	bool operator==(const Coord &other) const
	{
		return this->x == other.x && this->y == other.y;
	}

	Coord operator+(const Coord &other) const
	{
		return {this->x + other.x, this->y + other.y};
	}
};

// specific order - elves propose moves in this order, but rotating round
enum class Dir {
	North,
	South,
	West,
	East,
};

std::set<Coord> get_neighbours(const std::set<Coord> &elves, const Coord elf)
{
	std::set<Coord> neighbours;
	for (int y = -1; y <= 1; y++) {
		for (int x = -1; x <= 1; x++) {
			if (y == 0 && x == 0) continue;
			Coord neighbour{elf.x + x, elf.y + y};
			if (elves.find(neighbour) != elves.end()) {
				neighbours.insert(neighbour);
			}
		}
	}
	return neighbours;
}

std::set<Coord> play_round(const std::set<Coord> &starting_elves, int n)
{
	std::set<Coord> resulting_elves;
	std::map<Coord, std::vector<Coord>> proposed_moves; // new pos -> old pos (potentially many)
	for (const auto &elf : starting_elves) {
		const auto neighbours = get_neighbours(starting_elves, elf);
		if (neighbours.empty()) {
			// no neighbours, no movement
			resulting_elves.insert(elf);
		} else {
			// get proposed move
			// elves first proposed move rotates depending on the round number
			bool has_proposed_move = false;
			for (int proposed_dirn = n; proposed_dirn < n + 4; proposed_dirn++) {
				Dir proposed_dir = (Dir)(proposed_dirn % 4);
				const std::array<std::array<Coord, 3>, 4> dir_relative_neighbours {{
					{ Coord{-1, -1}, Coord{ 0, -1}, Coord{ 1, -1} }, // North
					{ Coord{-1,  1}, Coord{ 0,  1}, Coord{ 1,  1} }, // South
					{ Coord{-1, -1}, Coord{-1,  0}, Coord{-1,  1} }, // West
					{ Coord{ 1, -1}, Coord{ 1,  0}, Coord{ 1,  1} }, // East
				}};

				const std::array<Coord, 4> proposed_move_dir {{
					Coord{0, -1}, Coord{0, 1}, Coord{-1, 0}, Coord{1, 0} // NSWE
				}};

				bool can_move_here = true;
				for (const auto &n : dir_relative_neighbours[(int)proposed_dir]) {
					Coord neighbour = elf + n;
					if (neighbours.find(neighbour) != neighbours.end()) {
						// found existing elf, can't move in this direction
						can_move_here = false;
						break;
					}
				}
				if (can_move_here) {
					has_proposed_move = true;
					proposed_moves[elf + proposed_move_dir[(int)proposed_dir]].push_back(elf);
					break;
				}
			}
			if (!has_proposed_move) { // nowhere to move to :(
				resulting_elves.insert(elf);
			}
		}
	}

	// phase 2 - check proposals
	for (const auto &[new_pos, claimants] : proposed_moves) {
		if (claimants.size() == 1) {
			resulting_elves.insert(new_pos);
		} else {
			// multiple claimants, can't move
			for (const auto &old_pos : claimants) {
				resulting_elves.insert(old_pos);
			}
		}
	}
	assert(starting_elves.size() == resulting_elves.size());
	return resulting_elves;
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
"....#..\n"
"..###.#\n"
"#...#.#\n"
".#...##\n"
"#.###..\n"
"##.#.##\n"
".#..#..\n"
);

	std::set<Coord> elves;
	int y = 0;
	for (std::string line; std::getline(input, line); ) {
		for (int x = 0; x < (int)line.size(); x++) {
			if (line[x] == '#') {
				elves.insert(Coord{x, y});
			}
		}
		y++;
	}

	for (int i = 0; i < 10; i++) {
		elves = play_round(elves, i);
	}

	// determine rectangle
	Coord min{9999, 9999}, max{0, 0};
	for (const auto &elf : elves) {
		// May not necessarily be the same elf, so need to do each one individually
		min.x = std::min(min.x, elf.x);
		min.y = std::min(min.y, elf.y);
		max.x = std::max(max.x, elf.x);
		max.y = std::max(max.y, elf.y);
	}

	int total_area = (max.x - min.x + 1) * (max.y - min.y + 1);
	int num_free_spaces = total_area - elves.size();
	std::cout << "Number of total free spaces after 10 rounds: " << num_free_spaces << '\n';

	int round = 10;
	for (; ; round++) {
		std::set<Coord> new_elves = play_round(elves, round);
		if (new_elves == elves) break;
		elves = std::move(new_elves);
	}

	std::cout << "Number of rounds needed until the elves stop moving: " << round + 1 << '\n';
}
