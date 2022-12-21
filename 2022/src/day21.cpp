#include <algorithm>
#include <charconv>
#include <fstream>
#include <iostream>
#include <map>
#include <string_view>
#include <variant>

using MonkeyName = uint32_t;
MonkeyName parse_name(const std::string_view n)
{
	return n[0] << 24 | n[1] << 16 | n[2] << 8 | n[3];
}

std::string get_name(MonkeyName monkey)
{
	char c1 = (monkey >> 24) & 0xff;
	char c2 = (monkey >> 16) & 0xff;
	char c3 = (monkey >> 8) & 0xff;
	char c4 = monkey & 0xff;
	return std::string() + c1 + c2 + c3 + c4;
}

using Operation = std::tuple<MonkeyName, char, MonkeyName>;

struct Monkey {
	Monkey(MonkeyName name, int64_t number) : name(name), v(number) {}
	Monkey(MonkeyName name, Operation op) : name(name), v(op) {}
	MonkeyName name;
	std::variant<int64_t, Operation> v;
};

bool is_number(const std::string_view str)
{
	return std::all_of(str.begin(), str.end(), ::isdigit);
}

int64_t monkey_number(const std::map<MonkeyName, Monkey> &monkeys, const MonkeyName monkey)
{
	const auto &value = monkeys.at(monkey);
	if (std::holds_alternative<int64_t>(value.v)) {
		return std::get<int64_t>(value.v);
	}

	auto [monkey1, op, monkey2] = std::get<Operation>(value.v);
	auto monkey1_num = monkey_number(monkeys, monkey1);
	auto monkey2_num = monkey_number(monkeys, monkey2);
	switch (op) {
		case '+':
			return monkey1_num + monkey2_num;
		case '-':
			return monkey1_num - monkey2_num;
		case '*':
			return monkey1_num * monkey2_num;
		case '/':
			return monkey1_num / monkey2_num;
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

	std::map<MonkeyName, Monkey> monkeys;
	for (std::string line; std::getline(input, line); ) {
		std::string_view line_sv = line;
		MonkeyName name = parse_name(line_sv.substr(0, 4));
		std::string_view value = line.substr(6); // skips ': '
		int64_t number;
		const auto result = std::from_chars(value.data(), value.data() + value.size(), number);
		if (result.ec == std::errc()) {
			monkeys.try_emplace(name, name, number);
		} else {
			MonkeyName monkey1 = parse_name(value.substr(0, 4));
			char op = value[5];
			MonkeyName monkey2 = parse_name(value.substr(7));
			monkeys.try_emplace(name, name, Operation{monkey1, op, monkey2});
		}
	}

	std::cout << "Number yelled by monkey 'root': " << monkey_number(monkeys, parse_name("root")) << '\n';
