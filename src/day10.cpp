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

	std::string initial;
	std::getline(input, initial);
	std::string forty_res;

	for (int i = 0; i < 50; i++) {
		if (i == 40) forty_res = initial;
		std::string tmp;
		char cur = initial[0];
		int count = 0;
		for (auto c : initial) {
			if (c != cur) {
				tmp += (char)(count + 48);
				tmp += cur;
				count = 0;
				cur = c;
			}
			count++;
		}
		tmp += (char)(count + 48);
		tmp += cur;
		initial = tmp;
	}

	std::cout << "String length after 40 iterations: " << forty_res.size() << "\n";
	std::cout << "String length after 50 iterations: " << initial.size() << "\n";

	return 0;
}
