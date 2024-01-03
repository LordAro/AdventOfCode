#include <algorithm>
#include <charconv>
#include <cstdint>
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

// debugging
std::string get_name(MonkeyName monkey)
{
	char c1 = (monkey >> 24) & 0xff;
	char c2 = (monkey >> 16) & 0xff;
	char c3 = (monkey >> 8) & 0xff;
	char c4 = monkey & 0xff;
	return std::string() + c1 + c2 + c3 + c4;
}

using Operation = std::tuple<MonkeyName, char, MonkeyName>;
using Monkey = std::variant<int64_t, Operation>;

int64_t monkey_number(const std::map<MonkeyName, Monkey> &monkeys, const MonkeyName monkey)
{
	const auto &value = monkeys.at(monkey);
	if (std::holds_alternative<int64_t>(value)) {
		return std::get<int64_t>(value);
	}

	auto [monkey1, op, monkey2] = std::get<Operation>(value);
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

bool tree_contains(const std::map<MonkeyName, Monkey> &monkeys, MonkeyName target, MonkeyName root)
{
	if (root == target) {
		return true;
	}

	const auto &monkey = monkeys.at(root);
	if (std::holds_alternative<int64_t>(monkey))
	{
		return false;
	}

	const auto [root_monkey1, _, root_monkey2] = std::get<Operation>(monkey);
	return tree_contains(monkeys, target, root_monkey1) || tree_contains(monkeys, target, root_monkey2);
}

int64_t reverse_calc(const std::map<MonkeyName, Monkey> &monkeys, int64_t value, MonkeyName name)
{
	const MonkeyName HUMN = parse_name("humn");
	if (name == HUMN) {
		return value;
	}

	const auto [left_monkey, op, right_monkey] = std::get<Operation>(monkeys.at(name));
	bool left_contains_human = tree_contains(monkeys, HUMN, left_monkey);
	// we can just calculate the total from the side that doesn't include the number
	int64_t constant = monkey_number(monkeys, left_contains_human ? right_monkey : left_monkey);
	switch (op) {
		case '+':
			// V = X + C => X = V - C
			// V = C + X => X = V - C
			value -= constant;
			break;
		case '-':
			if (left_contains_human) {
				// V = X - C => X = V + C
				value += constant;
			} else {
				// V = C - X => -X = V - C => X = C - V
				value = constant - value;
			}
			break;
		case '*':
			// V = X * C => X = V / C
			// V = C * X => X = V / C
			value /= constant;
			break;
		case '/':
			if (left_contains_human) {
				// V = X / C => X = V * C
				value *= constant;
			} else {
				// V = C / X => V * X = C => X = C / V
				value = constant / value;
			}
			break;
		default:
			__builtin_unreachable();
	}

	return reverse_calc(monkeys, value, left_contains_human ? left_monkey : right_monkey);
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
		if (result.ec == std::errc()) { // no error
			monkeys.try_emplace(name, number);
		} else {
			MonkeyName monkey1 = parse_name(value.substr(0, 4));
			char op = value[5];
			MonkeyName monkey2 = parse_name(value.substr(7));
			monkeys.try_emplace(name, Operation{monkey1, op, monkey2});
		}
	}

	const MonkeyName ROOT = parse_name("root");
	const MonkeyName HUMN = parse_name("humn");
	std::cout << "Number yelled by monkey 'root': " << monkey_number(monkeys, ROOT) << '\n';

	const auto [root_monkey1, _, root_monkey2] = std::get<Operation>(monkeys.at(ROOT));
	bool found_human_in_left = tree_contains(monkeys, HUMN, root_monkey1);
	const MonkeyName target_monkey_subtree = found_human_in_left ? root_monkey2 : root_monkey1;
	const MonkeyName equation_monkey_subtree = found_human_in_left ? root_monkey1 : root_monkey2;
	const int64_t target_number = monkey_number(monkeys, target_monkey_subtree);

	std::cout << "Number needed to be yelled: " << reverse_calc(monkeys, target_number, equation_monkey_subtree) << '\n';
}
