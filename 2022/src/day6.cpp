#include <bitset>
#include <fstream>
#include <iostream>

template<int N>
bool is_substr_unique(const std::string &communication, size_t start_idx)
{
	std::bitset<26> set;
	for (size_t i = start_idx; i < start_idx + N; i++) {
		set.set(communication[i] - 'a'); // all lower-case
	}
	return set.count() == N;
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

	std::string line;
	input >> line;

	size_t start_of_packet = 0;
	size_t start_of_message = 0;
	for (size_t idx = 0; idx < line.size() - 4 && (start_of_packet == 0 || start_of_message == 0); idx++) {
		if (start_of_packet == 0 && is_substr_unique<4>(line, idx)) {
			start_of_packet = idx + 4;
		}
		if (start_of_message == 0 && is_substr_unique<14>(line, idx)) {
			start_of_message = idx + 14;
		}
	}

	std::cout << "Start-of-packet position marker: " << start_of_packet << '\n';
	std::cout << "Start-of-message position marker: " << start_of_message << '\n';
}

