#include <algorithm>
#include <deque>
#include <fstream>
#include <iostream>
#include <numeric>
#include <vector>

using OperationType = std::pair<char, int>;

struct Monkey {
	std::deque<int64_t> items;
	OperationType worryOp;
	int testDivN;
	int monkeyIdxTrue;
	int monkeyIdxFalse;

	int inspectCount = 0;
};

std::deque<int64_t> split_by(const std::string &s, char delim)
{
	std::deque<int64_t> res;

	size_t last = 0;
	size_t next = 0;
	while ((next = s.find(delim, last)) != std::string::npos) {
		res.push_back(std::stoi(s.substr(last, next-last)));
		last = next + 1;
	}
	res.push_back(std::stoi(s.substr(last)));
	return res;
}

std::istream &operator>>(std::istream &input, Monkey &m)
{
	std::string s;
	input >> s >> s; // Monkey N
	if (input.eof()) return input; // done
	input >> s >> s; // Starting items:
	std::getline(input, s); // remainder of line
	m.items = split_by(s, ',');
	input >> s >> s >> s >> s; // Operation: new = old

	input >> m.worryOp.first;
	input >> s;
	m.worryOp.second = s == "old" ? -1 : std::stoi(s);

	input >> s >> s >> s >> m.testDivN; // Test: divisible by N
	input >> s >> s >> s >> s >> s >> m.monkeyIdxTrue; // If true: throw to monkey N
	input >> s >> s >> s >> s >> s >> m.monkeyIdxFalse; // If false: throw to monkey N

	return input;
}

std::ostream &operator<<(std::ostream &output, Monkey &m)
{
	output << "Inspected: " << m.inspectCount << ' ';
	output << "Items (" << m.items.size() << "): (";
	for (const auto &i : m.items) output << i << ' ';
	output << ") ";
	output << "Op: " << m.worryOp.first << ' ' << m.worryOp.second;
	output << " Div test: " << m.testDivN << ' ' << m.monkeyIdxTrue << '/' << m.monkeyIdxFalse;
	return output;
}

std::vector<Monkey> play_round(const std::vector<Monkey> &input_monkeys, const int monkey_lcm)
{
	auto monkeys = input_monkeys;
	for (auto &m : monkeys) {
		while (!m.items.empty()) {
			m.inspectCount++;
			int64_t item = m.items.front();
			m.items.pop_front();

			// Monkey inspect
			int op = m.worryOp.second == -1 ? item : m.worryOp.second; // 'old'
			if (m.worryOp.first == '+') {
				item += op;
			}
			else if (m.worryOp.first == '*') {
				item *= op;
			} else {
				throw "Unknown worry operation";
			}

			// Monkey bored
			if (monkey_lcm == 0) {
				item /= 3;
			} else {
				item %= monkey_lcm;
			}

			// Monkey throw
			if (item % m.testDivN == 0) {
				monkeys[m.monkeyIdxTrue].items.push_back(item);
			} else {
				monkeys[m.monkeyIdxFalse].items.push_back(item);
			}
		}
	}
	return monkeys;
}

std::pair<int64_t, int64_t> get_inspector_monkeys(const std::vector<Monkey> &monkeys)
{
	std::vector<int> inspectCounts;
	for (const auto &m : monkeys) {
		inspectCounts.push_back(m.inspectCount);
	}
	std::sort(inspectCounts.begin(), inspectCounts.end());
	int mostInspected = *(inspectCounts.end() - 1);
	int nextMostInspected = *(inspectCounts.end() - 2);
	return {mostInspected, nextMostInspected};
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

	std::vector<Monkey> initial_monkeys;

	Monkey m;
	while (input >> m) {
		//std::cout << m << '\n';
		initial_monkeys.push_back(m);
	}

	auto non_worrying_monkeys = initial_monkeys;
	for (int round = 0; round < 20; round++) {
		non_worrying_monkeys = play_round(non_worrying_monkeys, 0);
		//std::cout << "After round " << round + 1 << '\n';
		//for (const auto &m : monkeys) {
		//	std::cout << m << '\n';
		//}
	}

	auto inspectors_p1 = get_inspector_monkeys(non_worrying_monkeys);
	std::cout << "After 20 rounds, the most active non-worrying monkeys inspected items " << inspectors_p1.first << " & " << inspectors_p1.second << " times"
	          << " - monkey business = " << inspectors_p1.first * inspectors_p1.second << '\n';

	auto worrying_monkeys = initial_monkeys;
	int monkey_lcm = std::accumulate(initial_monkeys.begin(), initial_monkeys.end(), 1, [](int t, const Monkey &m) { return t * m.testDivN; });
	for (int round = 0; round < 10'000; round++) {
		worrying_monkeys = play_round(worrying_monkeys, monkey_lcm);
	}
	auto inspectors_p2 = get_inspector_monkeys(worrying_monkeys);
	std::cout << "After 10000 rounds, the most active worrying monkeys inspected items " << inspectors_p2.first << " & " << inspectors_p2.second << " times"
	          << " - monkey business = " << inspectors_p2.first * inspectors_p2.second << '\n';
}
