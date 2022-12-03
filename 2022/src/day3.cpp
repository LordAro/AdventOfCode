#include <algorithm>
#include <fstream>
#include <iostream>
#include <set>
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

	std::vector<std::string> bags;

	std::string line;
	while (input >> line) {
		bags.push_back(line);
	}

	int priority_sum = 0;
	for (const auto &line : bags) {
		size_t half = line.length() / 2;
		std::set<char> compartment1; // set_intersection requires sorted ranges
		std::set<char> compartment2;

		compartment1.insert(line.begin(), line.begin() + half);
		compartment2.insert(line.begin() + half, line.end());

		std::string intersection;
		std::set_intersection(compartment1.begin(), compartment1.end(), compartment2.begin(), compartment2.end(), std::back_inserter(intersection));
		char duplicate = intersection[0];
		if (std::isupper(duplicate)) {
			priority_sum += duplicate - 'A' + 1 + 26;
		} else {
			priority_sum += duplicate - 'a' + 1;
		}
	}
	std::cout << "Duplicate item priority sum: " << priority_sum << '\n';

	int badge_priority_sum = 0;
	for (size_t i = 0; i < bags.size(); i += 3) {
		std::set<char> bag1(bags[i].begin(), bags[i].end());
		std::set<char> bag2(bags[i + 1].begin(), bags[i + 1].end());
		std::set<char> bag3(bags[i + 2].begin(), bags[i + 2].end());

		std::set<char> bag12;
		std::set_intersection(bag1.begin(), bag1.end(), bag2.begin(), bag2.end(), std::inserter(bag12, bag12.begin()));

		std::string intersection;
		std::set_intersection(bag12.begin(), bag12.end(), bag3.begin(), bag3.end(), std::back_inserter(intersection));
		char duplicate = intersection.at(0);
		if (std::isupper(duplicate)) {
			badge_priority_sum += duplicate - 'A' + 1 + 26;
		} else {
			badge_priority_sum += duplicate - 'a' + 1;
		}
	}
	std::cout << "Badge item priority sum: " << badge_priority_sum << '\n';
}
