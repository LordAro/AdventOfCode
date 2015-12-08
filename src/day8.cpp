#include <iostream>
#include <fstream>
#include <cassert>

int decoded_strlen(const std::string &str)
{
	int str_len = 0;
	bool skip_check = false;
	for (int i = 0; i < str.size(); i++) {
		if (skip_check) {
			str_len++;
			skip_check = false;
		} else if (str[i] == '\\' && i < str.size() - 3 && str[i + 1] == 'x') {
			str_len++;
			i += 3;
		} else if (str[i] == '\\') {
			skip_check = true;
		} else if (str[i] == '"') {
			// Skip
		} else {
			str_len++;
		}
	}
	return str_len;
}

int encoded_strlen(const std::string &str)
{
	int str_len = 2 + str.size(); // end quotes
	for (auto c : str) {
		if (c == '\\' || c == '"') str_len++;
	}
	return str_len;
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

	int code_len = 0;
	int decoded_len = 0;
	int encoded_len = 0;

	std::string line;
	while (std::getline(input, line)) {
		code_len += line.size();
		decoded_len += decoded_strlen(line);
		encoded_len += encoded_strlen(line);
	}
	std::cout << "Difference in decoded string lengths: " << code_len - decoded_len << "\n";
	std::cout << "Difference in encoded string lengths: " << encoded_len - code_len << "\n";

	return 0;
}
