#include <algorithm>
#include <fstream>
#include <iostream>
#include <map>
#include <string_view>

bool is_number(const std::string_view str)
{
	return std::all_of(str.begin(), str.end(), ::isdigit);
}

int64_t monkey_number(const std::map<std::string, std::string> &monkeys, const std::string_view monkey)
{
	const auto &value = monkeys.at(std::string(monkey)); // :(
	if (is_number(value)) {
		return std::stoi(value);
	}

	std::string_view sv = value;
	char op = sv[5];
	std::string_view monkey1 = sv.substr(0, 4);
	std::string_view monkey2 = sv.substr(7);
	switch (op) {
		case '+':
			return monkey_number(monkeys, monkey1) + monkey_number(monkeys, monkey2);
		case '-':
			return monkey_number(monkeys, monkey1) - monkey_number(monkeys, monkey2);
		case '*':
			return monkey_number(monkeys, monkey1) * monkey_number(monkeys, monkey2);
		case '/':
			return monkey_number(monkeys, monkey1) / monkey_number(monkeys, monkey2);
		default:
			__builtin_unreachable();
	}
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

	std::map<std::string, std::string> monkeys;
	for (std::string line; std::getline(input, line); ) {
		std::string name = line.substr(0, 4);
		std::string operation = line.substr(6);
		monkeys[name] = operation;
	}

	std::cout << "Number yelled by monkey 'root': " << monkey_number(monkeys, "root") << '\n';
}
