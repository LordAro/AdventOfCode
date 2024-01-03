#include <cstdint>
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

	int row_target = 0;
	int col_target = 0;

	std::string word;
	while (input >> word) {
		try {
			if (row_target == 0) {
				row_target = std::stoi(word);
			} else {
				col_target = std::stoi(word);
				break;
			}
		} catch (std::invalid_argument &) {
			// this is a hack
		}
	}

	std::cout << row_target << ' ' << col_target << '\n';

	uint64_t num = 20151125;

	// Diagonal iterator
	// Use 1 to skip first
	for (int k = 1; ; k++) {
		for (int col = 0; col <= k; col++) {
			int row = k - col;

			num = (num * 252533) % 33554393;

			if (col + 1 == col_target && row + 1 == row_target) {
				goto outer;
			}
		}
	}
outer:

	std::cout << "Code at position " << row_target << ',' << col_target << ": " << num << '\n';

	return 0;
}
