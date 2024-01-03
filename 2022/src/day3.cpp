#include <cstdint>
#include <fstream>
#include <iostream>
#include <vector>
#include <string_view>

#include <strings.h>

uint64_t priority(char c)
{
	if (c >= 'A' && c <= 'Z') {
		return c - 'A' + 1 + 26;
	}
	return c - 'a' + 1;
}

uint64_t priority_mask(std::string_view items)
{
	uint64_t out = 0;
	for (char c : items) {
		out |= 1ULL << priority(c);
	}
	return out;
}

int priority_from_mask(uint64_t mask)
{
	//std::bit_width
	return ffsll(static_cast<int64_t>(mask)) - 1;
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

	std::vector<std::string> bags;

	std::string bag;
	while (input >> bag) {
		bags.push_back(bag);
	}

	int priority_sum = 0;
	for (const auto &line : bags) {
		std::string_view line_view = line;
		size_t half = line_view.length() / 2;

		const auto compartment1 = line_view.substr(0, half);
		const auto compartment2 = line_view.substr(half);

		priority_sum += priority_from_mask(priority_mask(compartment1) & priority_mask(compartment2));
	}
	std::cout << "Duplicate item priority sum: " << priority_sum << '\n';

	int badge_priority_sum = 0;
	for (size_t i = 0; i < bags.size(); i += 3) {
		badge_priority_sum += priority_from_mask(priority_mask(bags[i]) & priority_mask(bags[i + 1]) & priority_mask(bags[i + 2]));
	}
	std::cout << "Badge item priority sum: " << badge_priority_sum << '\n';
}
