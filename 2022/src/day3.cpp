#include <algorithm>
#include <fstream>
#include <iostream>
#include <vector>

int priority(char c)
{
	if (std::isupper(c)) {
		return c - 'A' + 1 + 26;
	}
	return c - 'a' + 1;
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
		size_t half = line.length() / 2;

		std::string compartment1(line.begin(), line.begin() + half);
		std::string compartment2(line.begin() + half, line.end());
		std::sort(compartment1.begin(), compartment1.end());
		std::sort(compartment2.begin(), compartment2.end());

		std::string intersection;
		std::set_intersection(compartment1.begin(), compartment1.end(), compartment2.begin(), compartment2.end(), std::back_inserter(intersection));
		priority_sum += priority(intersection[0]); // only 1!
	}
	std::cout << "Duplicate item priority sum: " << priority_sum << '\n';

	int badge_priority_sum = 0;
	for (size_t i = 0; i < bags.size(); i += 3) {
		std::string bag1 = bags[i];
		std::string bag2 = bags[i + 1];
		std::string bag3 = bags[i + 2];
		std::sort(bag1.begin(), bag1.end());
		std::sort(bag2.begin(), bag2.end());
		std::sort(bag3.begin(), bag3.end());

		std::string bag12;
		std::set_intersection(bag1.begin(), bag1.end(), bag2.begin(), bag2.end(), std::back_inserter(bag12));

		std::string intersection;
		std::set_intersection(bag12.begin(), bag12.end(), bag3.begin(), bag3.end(), std::back_inserter(intersection));
		badge_priority_sum += priority(intersection[0]);
	}
	std::cout << "Badge item priority sum: " << badge_priority_sum << '\n';
}
