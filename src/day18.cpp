#include <iostream>
#include <fstream>
#include <vector>
#include <algorithm>

std::vector<std::string> do_gol_step(const std::vector<std::string> &initial)
{
	std::vector<std::string> grid = initial;

	for (int i = 0; i < (int)initial.size(); i++) {
		for (int j = 0; j < (int)initial[i].size(); j++) {
			// Get neighbours
			int on_neighbours = 0;
			for (int x = std::max(0, i-1); x <= std::min(i+1, (int)initial.size() - 1); x++) {
				for (int y = std::max(0, j-1); y <= std::min(j+1, (int)initial[i].size() - 1); y++) {
					if (x == i && y == j) continue;
					if (initial[x][y] == '#') on_neighbours++;
				}
			}

			if (initial[i][j] == '#' && (on_neighbours != 2 && on_neighbours != 3)) grid[i][j] = '.';
			if (initial[i][j] == '.' && on_neighbours == 3) grid[i][j] = '#';
		}
	}
	return grid;
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

	std::vector<std::string> initial;

	std::string line;
	while (std::getline(input, line)) {
		initial.push_back(line);
	}

	int num_steps = 100;
	std::vector<std::string> grid = initial;

	for (int i = 0; i < num_steps; i++) {
		grid = do_gol_step(grid);
	}

	int num_lights = 0;
	for (const auto &line : grid) num_lights += std::count(line.begin(), line.end(), '#');
	std::cout << "Number of lights turned on after 100 steps: " << num_lights << "\n";

	// Part 2
	grid = initial;
	std::vector<std::pair<int, int>> perma_on = {{0, 0}, {grid.size()-1, 0}, {0, grid.size()-1}, {grid.size()-1, grid.size()-1}};
	for (int i = 0; i < num_steps; i++) {
		for (const auto &point : perma_on) grid[point.first][point.second] = '#';
		grid = do_gol_step(grid);
	}
	// Reapply broken lights, in case they got turned off by last step
	for (const auto &point : perma_on) grid[point.first][point.second] = '#';

	num_lights = 0;
	for (const auto &line : grid) num_lights += std::count(line.begin(), line.end(), '#');
	std::cout << "Number of lights turned on after 100 steps (with faulty lights): " << num_lights << "\n";

	return 0;
}
