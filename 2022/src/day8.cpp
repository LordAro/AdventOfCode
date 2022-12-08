#include <algorithm>
#include <fstream>
#include <iostream>
#include <vector>

bool is_visible(const std::vector<std::vector<int>> &grid, const size_t startx, const size_t starty)
{
	const int my_height = grid[starty][startx];
	//std::cout << "Pos: " << startx << ", " << starty << " (" << my_height << ")\n";

	bool is_visible_left = true;
	bool is_visible_right = true;
	bool is_visible_above = true;
	bool is_visible_below = true;

	for (size_t x = startx; x > 0; x--) {
		// use offset by one to avoid overflow issues
		if (grid[starty][x - 1] >= my_height) {
			//std::cout << "  not visible from left - " << x - 1 << ", " << starty << " (" << grid[starty][x - 1] << ")\n";
			is_visible_left = false;
			break;
		}
	}
	for (size_t x = startx + 1; x < grid[starty].size(); x++) {
		if (grid[starty][x] >= my_height) {
			//std::cout << "  not visible from right - " << x << ", " << starty << " (" << grid[starty][x] << ")\n";
			is_visible_right = false;
			break;
		}
	}
	for (size_t y = starty; y > 0; y--) {
		if (grid[y - 1][startx] >= my_height) {
			//std::cout << "  not visible from above - " << startx << ", " << y - 1 << " (" << grid[y - 1][startx] << ")\n";
			is_visible_above = false;
			break;
		}
	}
	for (size_t y = starty + 1; y < grid.size(); y++) {
		if (grid[y][startx] >= my_height) {
			//std::cout << "  not visible from below - " << startx << ", " << y << " (" << grid[y][startx] << ")\n";
			is_visible_below = false;
			break;
		}
	}

	return is_visible_left || is_visible_right || is_visible_above || is_visible_below;
}

int scenic_score(const std::vector<std::vector<int>> &grid, const size_t startx, const size_t starty)
{
	const int my_height = grid[starty][startx];

	int num_visible_left = 0;
	int num_visible_right = 0;
	int num_visible_above = 0;
	int num_visible_below = 0;

	for (size_t x = startx; x > 0; x--) {
		num_visible_left++;
		// use offset by one to avoid overflow issues
		if (grid[starty][x - 1] >= my_height) {
			break;
		}
	}
	for (size_t x = startx + 1; x < grid[starty].size(); x++) {
		num_visible_right++;
		if (grid[starty][x] >= my_height) {
			break;
		}
	}
	for (size_t y = starty; y > 0; y--) {
		num_visible_above++;
		if (grid[y - 1][startx] >= my_height) {
			break;
		}
	}
	for (size_t y = starty + 1; y < grid.size(); y++) {
		num_visible_below++;
		if (grid[y][startx] >= my_height) {
			break;
		}
	}

	return num_visible_left * num_visible_right * num_visible_above * num_visible_below;
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

	std::vector<std::vector<int>> grid;

	std::string line;
	while (std::getline(input, line)) {
		if (line.empty()) continue;
		std::vector<int> row;
		std::transform(line.cbegin(), line.cend(), std::back_inserter(row), [](unsigned char c) { return c - '0'; } );
		grid.push_back(row);
	}

//	grid = {
//		{ 3, 0, 3, 7, 3 },
//		{ 2, 5, 5, 1, 2 },
//		{ 6, 5, 3, 3, 2 },
//		{ 3, 3, 5, 4, 9 },
//		{ 3, 5, 3, 9, 0 },
//	};

	size_t visible_count = 0;
	int best_scenic_score = 0;

	// Skip edges
	for (size_t y = 1; y < grid.size() - 1; y++) {
		for (size_t x = 1; x < grid[y].size() - 1; x++) {
			visible_count += is_visible(grid, x, y);
			best_scenic_score = std::max(best_scenic_score, scenic_score(grid, x, y));
		}
	}

	// Add edges
	visible_count += grid[0].size() + grid[0].size() + grid.size() - 2 + grid.size() - 2;
	std::cout << "Number of trees visible from outside: " << visible_count << '\n';
	std::cout << "Best scenic score: " << best_scenic_score << '\n';
}
