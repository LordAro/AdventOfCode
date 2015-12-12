#include <iostream>
#include <fstream>

std::string get_next_pw(std::string cur)
{
	for (size_t i = cur.size(); i-- > 0; ) {
		if (cur[i] == 'z') {
			cur[i] = 'a';
			continue;
		} else {
			cur[i]++;
			break;
		}
	}
	return cur;
}

bool has_increasing(const std::string &pw, int len)
{
	for (size_t i = 0; i < (pw.size() - len + 1); i++) {
		bool breaked = false;
		for (int j = 1; j < len; j++) {
			if (pw[i] + j != pw[i + j]) {
				breaked = true;
				break;
			}
		}
		if (!breaked) {
			return true;
		}
	}
	return false;
}

bool contains_any_of(const std::string &line, const std::initializer_list<std::string> &list)
{
	for (const auto &item : list) {
		if (line.find(item) != std::string::npos) return true;
	}
	return false;
}

bool contains_separate_pairs(const std::string &line, int num)
{
	std::string found_chrs;
	for (size_t i = 0; i < line.size() - 1; i++) {
		if (line[i] == line[i + 1] && found_chrs.find(line[i] == std::string::npos)) {
			found_chrs += line[i];
			i++; // Non overlapping
		}
	}
	return found_chrs.size() >= num;
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

	std::string current_pw;
	std::string new_pw;
	input >> current_pw;

	bool valid = false;
	while (!valid) {
		current_pw = get_next_pw(current_pw);
		valid = has_increasing(current_pw, 3)
			&& !contains_any_of(current_pw, {"i", "o", "l"})
			&& contains_separate_pairs(current_pw, 2);
		if (valid && new_pw.size() == 0) {
			new_pw = current_pw;
			valid = false;
		}
	}

	std::cout << "Santa's next password: " << new_pw << "\n";
	std::cout << "Santa's next next password: " << current_pw << "\n";


	return 0;
}
