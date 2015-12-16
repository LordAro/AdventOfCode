#include <iostream>
#include <fstream>
#include <regex>

struct Sue {
	int num;
	int children;
	int cats;
	int samoyeds;
	int pomeranians;
	int akitas;
	int vizslas;
	int goldfish;
	int trees;
	int cars;
	int perfumes;
};

bool attr_match(const Sue &target, const std::string &attr, int val)
{
	if (attr == "children") {
		return target.children == val;
	} else if (attr == "cats") {
		return target.cats == val;
	} else if (attr == "samoyeds") {
		return target.samoyeds == val;
	} else if (attr == "pomeranians") {
		return target.pomeranians == val;
	} else if (attr == "akitas") {
		return target.akitas == val;
	} else if (attr == "vizslas") {
		return target.vizslas == val;
	} else if (attr == "goldfish") {
		return target.goldfish == val;
	} else if (attr == "trees") {
		return target.trees == val;
	} else if (attr == "cars") {
		return target.cars == val;
	} else if (attr == "perfumes") {
		return target.perfumes == val;
	}
	return false;
}

// Exactly the same as above, but with part 2 "ruleset" - some equals replaced with lt/gt
bool attr_match2(const Sue &target, const std::string &attr, int val)
{
	if (attr == "children") {
		return target.children == val;
	} else if (attr == "cats") {
		return target.cats < val;
	} else if (attr == "samoyeds") {
		return target.samoyeds == val;
	} else if (attr == "pomeranians") {
		return target.pomeranians > val;
	} else if (attr == "akitas") {
		return target.akitas == val;
	} else if (attr == "vizslas") {
		return target.vizslas == val;
	} else if (attr == "goldfish") {
		return target.goldfish > val;
	} else if (attr == "trees") {
		return target.trees < val;
	} else if (attr == "cars") {
		return target.cars == val;
	} else if (attr == "perfumes") {
		return target.perfumes == val;
	}
	return false;
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

	Sue target = {0, 3, 7, 2, 3, 0, 0, 5, 3, 2, 1};

	std::string line;
	while (std::getline(input, line)) {
		std::regex_match(line, match, base_regex);

		if (attr_match(target, match[2], std::stoi(match[3])) &&
				attr_match(target, match[4], std::stoi(match[5])) &&
				attr_match(target, match[6], std::stoi(match[7]))) {
			std::cout << "Found matching aunt: Sue " << match[1] << "\n";
		}

		if (attr_match2(target, match[2], std::stoi(match[3])) &&
				attr_match2(target, match[4], std::stoi(match[5])) &&
				attr_match2(target, match[6], std::stoi(match[7]))) {
			std::cout << "Found 2nd matching aunt: Sue " << match[1] << "\n";
		}
	}

	return 0;
}
