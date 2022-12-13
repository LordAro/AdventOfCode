#include <cassert>
#include <fstream>
#include <iostream>
#include <sstream>
#include <variant>
#include <vector>

struct IntListNode {
	std::variant<int, std::vector<IntListNode>> item;
};

std::ostream &operator<<(std::ostream &os, const IntListNode &node)
{
	if (std::holds_alternative<int>(node.item)) {
		os << std::get<int>(node.item);
	} else {
		os << '[';
		for (const auto &n : std::get<std::vector<IntListNode>>(node.item)) {
			os << n << ',';
		}
		os << ']';
	}
	return os;
}

std::istream &operator>>(std::istream &is, IntListNode &node)
{
	char c = (is >> std::ws).peek();
	if (c == '[') {
		is.get();
		std::vector<IntListNode> nl;
		while (is.good()) {
			c = is.peek();
			switch (c) {
				case ',':
				case ']':
					is.get();
					break;
				default: {
					IntListNode n;
					is >> n;
					nl.push_back(n);
					break;
				}
			}
			if (c == ']') {
				node.item = nl;
				break;
			}
		}
	} else {
		int val;
		is >> val;
		node.item = val;
	}
	return is;
}

// Could be be <=> in C++20
// -1 == a < b
//  0 == a == b
// +1 == a > b
int cmp(const IntListNode &left, const IntListNode &right)
{
	std::cout << "Comparing " << left << " & " << right << '\n';
	if (left.item.index() == right.item.index()) {
		if (std::holds_alternative<int>(left.item)) {
			// if ints equal, continue onto the next. For our purposes, this is equivalent to <=
			const auto &left_int = std::get<int>(left.item);
			const auto &right_int = std::get<int>(right.item);
			int ret = 0;
			if (left_int != right_int) {
				ret = left_int < right_int ? -1 : 1;
			}
			std::cout << " => ret=" << ret << '\n';
			return ret;
		}
		const auto &left_list = std::get<std::vector<IntListNode>>(left.item);
		const auto &right_list = std::get<std::vector<IntListNode>>(right.item);
		size_t idx = 0;
		for (; idx < left_list.size() && idx < right_list.size(); idx++) {
			int ret = cmp(left_list[idx], right_list[idx]);
			if (ret == 0) continue;
			std::cout << " => ret2=" << ret << '\n';
			return ret;
		}
		if (idx == left_list.size() && idx != right_list.size()) {
			std::cout << " => left out of items\n";
			return -1; // If the left list runs out of items first, the inputs are in the right order
		} else if (idx == right_list.size() && idx != left_list.size()) {
			std::cout << " => right out of items\n";
			return 1;
		}
		return 0;
	} else if (std::holds_alternative<int>(left.item)) {
		std::vector<IntListNode> vec;
		vec.push_back(left);
		IntListNode fakelist;
		fakelist.item = vec;
		int ret = cmp(fakelist, right);
		std::cout << " => ret3=" << ret << '\n';
		return ret;
	} else {
		std::vector<IntListNode> vec;
		vec.push_back(right);
		IntListNode fakelist;
		fakelist.item = vec;
		int ret = cmp(left, fakelist);
		std::cout << " => ret4=" << ret << '\n';
		return ret;
	}
	// unreachable
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

	std::string example_text =
		"[1,1,3,1,1]\n"
		"[1,1,[5],1,1]\n"
		"[[1],[2,3,4]]\n"
		"[[1],4]\n"
		"\n"
		"[9]\n"
		"[[8,7,6]]\n"
		"\n"
		"[[4,4],4,4]\n"
		"[[4,4],4,4,4]\n"
		"\n"
		"[7,7,7,7]\n"
		"[7,7,7]\n"
		"\n"
		"[]\n"
		"[3]\n"
		"\n"
		"[[[]]]\n"
		"[[]]\n"
		"\n"
		"[1,[2,[3,[4,[5,6,7]]]],8,9]\n"
		"[1,[2,[3,[4,[5,6,0]]]],8,9]\n"
	;
	std::stringstream ss(example_text);

	int ordered_pair_index_sum = 0;

	int pair_index = 1;
	IntListNode pd1, pd2;
	while (input >> pd1 >> pd2) {
	//while (ss >> pd1 >> pd2) {
		if (cmp(pd1, pd2) == -1) ordered_pair_index_sum += pair_index;
		pair_index++;
	}
	std::cout << "Ordered pair index sum: " << ordered_pair_index_sum << '\n';
}
