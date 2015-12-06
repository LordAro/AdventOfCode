#include <iostream>
#include <fstream>

int main()
{
	std::fstream input("day1.input");
	if (!input.is_open()) {
		std::cout << "Could not open file\n";
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
