#include <fstream>
#include <iostream>

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

	int contained_within = 0;
	int overlap_count = 0;
	char c;
	std::pair<int, int> elf1, elf2;
	while(input >> elf1.first >> c >> elf1.second >> c >> elf2.first >> c >> elf2.second) {
		contained_within +=
			(elf1.first >= elf2.first && elf1.second <= elf2.second)
			|| (elf2.first >= elf1.first && elf2.second <= elf1.second);

		overlap_count +=
			(elf1.first >= elf2.first && elf1.first <= elf2.second)
			|| (elf1.second >= elf2.first && elf1.second <= elf2.second)
		    || (elf2.first >= elf1.first && elf2.first <= elf1.second)
			|| (elf2.second >= elf1.first && elf2.second <= elf1.second);
	}
	std::cout << "Pairs contained within others: " << contained_within << '\n';
	std::cout << "Overlapping pairs: " << overlap_count << '\n';
}

