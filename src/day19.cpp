#include <iostream>
#include <fstream>
#include <sstream>
#include <map>
#include <vector>
#include <set>
#include <cctype>

using map_t = std::map<std::string, std::vector<std::string>>;

std::set<std::string> get_all_replacements(const map_t &conversions, const std::string &input)
{
	std::set<std::string> list;
	for (size_t i = 0; i < input.size(); i++) {
		if (islower(input[i])) continue;

		std::string elem(1, input[i]);
		if (i < input.size() - 1 && islower(input[i + 1])) elem += input[i + 1];
		auto elem_convs = conversions.find(elem);
		if (elem_convs == conversions.end()) continue;

		for (const auto &conv : elem_convs->second) {
			std::string converted = input;
			converted.replace(i, elem.size(), conv);
			list.insert(converted);
		}
	}
	return list;
}

int fabrication_count(const map_t &conversions, const std::string &input)
{
	if (input == "") return 0;
	for (const auto &conv : conversions) {
		size_t pos;
		size_t len = 1;
		for (const auto &res : conv.second) {
			pos = input.find(res);
			if (pos != std::string::npos) {
				len = res.size();
				break;
			}
		}
		if (pos == std::string::npos) continue;

		std::string fab = input;
		std::string repl = conv.first == "e" ? "" : conv.first;
		fab.replace(pos, len, repl);
		return fabrication_count(conversions, fab) + 1;
	}
	return 999999; // No result
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

	auto list = get_all_replacements(conversions, medicine);
	std::cout << "Number of possible replacements: " << list.size() << "\n";

	std::cout << "Fabrication count: " << fabrication_count(conversions, medicine) << "\n";
	return 0;
}
