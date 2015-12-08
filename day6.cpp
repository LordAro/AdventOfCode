#include <iostream>
#include <fstream>
#include <array>
#include <sstream>
#include <tuple>

std::pair<int, int> get_coord(const std::string &coordstr)
{
	std::istringstream a(coordstr);
	char comma;
	int x, y;
	a >> x >> comma >> y;
	return std::make_pair(x, y);
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

	std::array<std::array<bool, 1000>, 1000> grid{};
	std::array<std::array<int, 1000>, 1000> grid_new{};

	std::string line;
	while (std::getline(input, line)) {
		int start = 0;
		if (line.find("turn on") == 0) {
			start = 8;
		} else if (line.find("turn off") == 0) {
			start = 9;
		} else if (line.find("toggle") == 0) {
			start = 7;
		}
		std::istringstream coordstr(line.substr(start, line.size()-start));
		std::string first, through, second;
		coordstr >> first >> through >> second;

		int x1, y1, x2, y2;
		std::tie(x1, y1) = get_coord(first);
		std::tie(x2, y2) = get_coord(second);

		for (int x = x1; x <= x2; x++) {
			for (int y = y1; y <= y2; y++) {
				switch (start) {
					case 7:
						grid[x][y] = !grid[x][y];
						grid_new[x][y] += 2;
						break;
					case 8:
						grid[x][y] = true;
						grid_new[x][y] += 1;
						break;
					case 9:
						grid[x][y] = false;
						grid_new[x][y] = std::max(grid_new[x][y] - 1, 0);
						break;
				}
			}
		}
	}
	int count = 0;
	int total_brightness = 0;
	for (int i = 0; i < 1000; i++) {
		for (int j = 0; j < 1000; j++) {
			if (grid[i][j]) count++;
			total_brightness += grid_new[i][j];
		}
	}

	std::cout << "Total lights lit: " << count << "\n";
	std::cout << "Total brightness: " << total_brightness << "\n";
	return 0;
}
