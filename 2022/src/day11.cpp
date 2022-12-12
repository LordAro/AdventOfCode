#include <algorithm>
#include <cassert>
#include <fstream>
#include <iostream>
#include <numeric>
#include <unordered_map>
#include <vector>

using OperationType = std::pair<char, int>;

struct Monkey {
	std::vector<int64_t> items;
	OperationType worryOp;
	int testDivN;
	int monkeyIdxTrue;
	int monkeyIdxFalse;

	int inspectCount = 0;
};

bool operator==(const Monkey &a, const Monkey &b)
{
	return a.testDivN == b.testDivN && a.items == b.items;
}

template <class T>
inline void hash_combine(std::size_t& seed, T const& v)
{
    seed ^= std::hash<T>()(v) + 0x9e3779b9 + (seed << 6) + (seed >> 2);
}

namespace std {
template<>
struct hash<Monkey> {
	size_t operator()(const Monkey &m) const
	{
		std::size_t seed = m.items.size();
		for (auto& i : m.items) {
			seed ^= i + 0x9e3779b9 + (seed << 6) + (seed >> 2);
		}
		return seed ^ (std::hash<int>()(m.testDivN) << 1);
	}
};

template<>
struct hash<std::vector<Monkey>> {
	size_t operator()(const std::vector<Monkey> &mvec) const
	{
		size_t seed = 0;
		for (size_t i = 0; i < mvec.size(); i++) {
			// Combine the hash of the current vector with the hashes of the previous ones
			hash_combine(seed, mvec[i]);
		}
		return seed;
	}
};
}

std::vector<int64_t> split_by(const std::string &s, char delim)
{
	std::vector<int64_t> res;

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

// Debugging
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

void play_round(std::vector<Monkey> &monkeys, const int monkey_lcm)
{
	for (auto &m : monkeys) {
		for (int64_t item : m.items) {
			m.inspectCount++;

			// Monkey inspect
			int64_t operand = m.worryOp.second == -1 ? item : m.worryOp.second; // 'old'
			if (m.worryOp.first == '+') {
				item += operand;
			}
			else if (m.worryOp.first == '*') {
				item *= operand;
			} else {
				__builtin_unreachable();
			}

			// Monkey bored
			if (monkey_lcm == 0) {
				item /= 3;
			} else {
				item %= monkey_lcm;
			}

			// Monkey throw
			int monkeyTarget = item % m.testDivN == 0 ? m.monkeyIdxTrue : m.monkeyIdxFalse;
			monkeys[monkeyTarget].items.push_back(item);
		}
		m.items.clear();
	}
}

std::pair<int64_t, int64_t> get_inspector_monkeys(const std::vector<Monkey> &monkeys)
{
	std::vector<int> inspectCounts;
	std::transform(monkeys.cbegin(), monkeys.cend(), std::back_inserter(inspectCounts), [](const auto &m) { return m.inspectCount; });
	std::sort(inspectCounts.rbegin(), inspectCounts.rend()); // reverse sort
	int mostInspected = *inspectCounts.begin();
	int nextMostInspected = *(inspectCounts.begin() + 1);
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
		initial_monkeys.push_back(m);
	}

	auto non_worrying_monkeys = initial_monkeys;
	for (int round = 0; round < 20; round++) {
		play_round(non_worrying_monkeys, 0);
	}

	auto inspectors_p1 = get_inspector_monkeys(non_worrying_monkeys);
	std::cout << "After 20 rounds, the most active non-worrying monkeys inspected items " << inspectors_p1.first << " & " << inspectors_p1.second << " times"
	          << " - monkey business = " << inspectors_p1.first * inspectors_p1.second << '\n';

	auto worrying_monkeys = initial_monkeys;

	// All testDivN are co-prime, so this works as lcm
	int monkey_lcm = std::accumulate(initial_monkeys.begin(), initial_monkeys.end(), 1, [](int t, const Monkey &m) { return t * m.testDivN; });

	// Possible optimisation: cycle detection. But the cost of finding, storing and comparing
	// monkey states is too much compared to just doing the full loop
	std::vector<std::vector<Monkey>> monkey_cache;
	monkey_cache.push_back(worrying_monkeys);
	size_t cycle_start = 0;
	size_t cycle_end = 0;
	for (int round = 0; round < 10'000; round++) {
		play_round(worrying_monkeys, monkey_lcm);
		monkey_cache.push_back(worrying_monkeys);
		if (auto cycle_iterator = std::find(monkey_cache.begin(), monkey_cache.end() - 1, worrying_monkeys); cycle_iterator != monkey_cache.end() - 1) {
			cycle_start = std::distance(monkey_cache.begin(), cycle_iterator);
			cycle_end = round; // +1 ?
			assert(monkey_cache[cycle_start] == monkey_cache[cycle_end + 1]);
			break;
		}
	}

	{
		size_t cycle_length = cycle_end - cycle_start;
		int mod_idx = (10000 % (cycle_length));
		std::cout << cycle_start << '-' << cycle_end << " = " << cycle_length << '(' << mod_idx << ")\n";

		std::vector<int> inspection_increases;
		for (size_t i = 0; i < initial_monkeys.size(); i++) {
			inspection_increases.push_back(monkey_cache[cycle_end][i].inspectCount - monkey_cache[cycle_start][i].inspectCount);
		}

		std::vector<int64_t> inspection_counts10k;
		for (size_t i = 0; i < initial_monkeys.size(); i++) {
			inspection_counts10k.push_back(worrying_monkeys[mod_idx].inspectCount + 4 * inspection_increases[i]); // XXX hardcoded 4
		}
		std::sort(inspection_counts10k.rbegin(), inspection_counts10k.rend()); // reverse sort
		std::cout << "Foo: " << *inspection_counts10k.begin() << ' ' << *(inspection_counts10k.begin() + 1) << " = " << *inspection_counts10k.begin() * *(inspection_counts10k.begin() + 1) << '\n';
	}

//	auto inspectors_p2 = get_inspector_monkeys(worrying_monkeys);
//	std::cout << "After 10000 rounds, the most active worrying monkeys inspected items " << inspectors_p2.first << " & " << inspectors_p2.second << " times"
//	          << " - monkey business = " << inspectors_p2.first * inspectors_p2.second << '\n';
}
