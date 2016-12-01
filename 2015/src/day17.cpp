#include <iostream>
#include <fstream>
#include <vector>
#include <algorithm>

std::vector<std::vector<int>> get_combinations(int sum, std::vector<int> containers)
{
	if (sum <= 0) return {};

	std::vector<std::vector<int>> list;
	for (size_t i = 0; i < containers.size(); i++) {
		int cont = containers[i];
		if (cont == sum) {
			list.push_back({cont});
			continue;
		}

		std::vector<int> subset(containers.begin() + i + 1, containers.end());
		auto tmp = get_combinations(sum - cont, subset);
		for (auto arr : tmp) {
			arr.push_back(cont);
			list.push_back(arr);
		}
	}
	return list;
}

#include <sstream>

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

	std::vector<int> containers;

	int num;
	while (input >> num) {
		containers.push_back(num);
	}

	int total = 150;
	auto combinations = get_combinations(total, containers);
	std::cout << "Number of different combinations: " << combinations.size() << "\n";

	size_t minsize = containers.size();
	for (const auto &comb : combinations) {
		minsize = std::min(comb.size(), minsize);
	}
	int min_slns = std::count_if(combinations.begin(), combinations.end(), [minsize](const std::vector<int> &a){return a.size() == minsize;});
	std::cout << "Number of minimal combinations: " << min_slns << "\n";

	return 0;
}
