#include <iostream>
#include <fstream>
#include <sstream>
#include <map>
#include <vector>

using map_t = std::map<std::string, std::vector<std::string>>;

std::vector<std::string> get_all_replacements(map_t conversions, std::string input)
{
	for (const auto &conv : conversions) {
		for (const auto &res : conv.second) {
			auto it = input.find(conv.first);
		}
	}
}

int main(int argc, char **argv)
{
	if (argc != 2) {
		std::cerr << "Incorrect number of arguments provided\n";
		return 1;
	}
//	std::fstream input(argv[1]);
//	if (!input) {
//		std::cerr << "Could not open input file\n";
//		return 1;
//	}
	std::istringstream input("H => HO\nH => OH\nO => HH\n\n HOH");

	map_t conversions;

	std::string line;
	while (std::getline(input, line)) {
		if (line == "") break;

		std::istringstream stream(line);
		std::string key, arrow, value;
		stream >> key >> arrow >> value;
		conversions[key].push_back(value);
	}

	std::string medicine;
	std::getline(input, medicine);
	std::cout << medicine << "\n";

	for (const auto &it : conversions) {
		std::cout << it.first << " => ";
		for (const auto &res : it.second) std::cout << res << ",";
		std::cout << "\n";
	}

	return 0;
}
