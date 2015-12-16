#include <iostream>
#include <fstream>
#include <regex>

// Part2 "ruleset"
bool part2_match(const std::map<std::string, int> &target, const std::string &attr, int val)
{
	if (attr == "cats" || attr == "trees") {
		return target.at(attr) < val;
	} else if (attr == "pomeranians" || attr == "goldfish") {
		return target.at(attr) > val;
	} else {
		return target.at(attr) == val;
	}
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

	std::regex base_regex(R"(Sue (\S+): (\S+): (\S+), (\S+): (\S+), (\S+): (\S+))");
	std::smatch match;

	std::map<std::string, int> target {
		{"children",    3},
		{"cats",        7},
		{"samoyeds",    2},
		{"pomeranians", 3},
		{"akitas",      0},
		{"vizslas",     0},
		{"goldfish",    5},
		{"trees",       3},
		{"cars",        2},
		{"perfumes",    1},
	};

	std::string line;
	while (std::getline(input, line)) {
		std::regex_match(line, match, base_regex);

		if (target[match[2]] == std::stoi(match[3]) &&
				target[match[4]] == std::stoi(match[5]) &&
				target[match[6]] == std::stoi(match[7])) {
			std::cout << "Found matching aunt: Sue " << match[1] << "\n";
		}

		if (part2_match(target, match[2], std::stoi(match[3])) &&
				part2_match(target, match[4], std::stoi(match[5])) &&
				part2_match(target, match[6], std::stoi(match[7]))) {
			std::cout << "Found 2nd matching aunt: Sue " << match[1] << "\n";
		}
	}

	return 0;
}
