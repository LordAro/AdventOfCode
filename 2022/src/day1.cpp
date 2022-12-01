#include <algorithm>
#include <fstream>
#include <iostream>
#include <vector>

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

	std::vector<int> calorie_sums;
	int cal_sum = 0;
	std::string line;
	while (std::getline(input, line)) {
		if (line.empty()) {
			calorie_sums.push_back(cal_sum);
			cal_sum = 0;
		} else {
			cal_sum += std::stoi(line);
		}
	}
	calorie_sums.push_back(cal_sum);
	std::sort(calorie_sums.rbegin(), calorie_sums.rend());

	std::cout << "Elf carrying the most calories is carrying: " << calorie_sums[0] << '\n';
	std::cout << "Top three elves carrying the most calories total: " << calorie_sums[0] + calorie_sums[1] + calorie_sums[2] << '\n';
}
