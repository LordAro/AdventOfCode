#include <iostream>
#include <fstream>

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

	int floor = 0;
	int count = 0;
	int basement_pos = 0;
	char c;
	while (input >> c) {
		count++;
		if (c == '(') {
			floor++;
		} else if (c == ')') {
			floor--;
		}
		if (basement_pos == 0 && floor < 0) {
			basement_pos = count;
		}
	}

	std::cout << "Santa is on floor: " << floor << "\n";
	std::cout << "Santa entered the basement on move: " << basement_pos << "\n";
	return 0;
}
