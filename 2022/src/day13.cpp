#include <algorithm>
#include <fstream>
#include <iostream>
#include <sstream>
#include <variant>
#include <vector>

struct IntListNode : std::variant<int, std::vector<IntListNode>>  {
	using variant::variant;
};

std::ostream &operator<<(std::ostream &os, const IntListNode &node)
{
	if (std::holds_alternative<int>(node)) {
		os << std::get<int>(node);
	} else {
		os << '[';
		for (const auto &n : std::get<std::vector<IntListNode>>(node)) {
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
				node = nl;
				break;
			}
		}
	} else {
		int val;
		is >> val;
		node = val;
	}
	return is;
}

template<bool equal>
struct IntListNode_Comparison_Visitor {
	bool operator()(int left, int right)
	{
		if constexpr (equal) {
			return left == right;
		} else {
			return left < right;
		}
	}

	bool operator()(const std::vector<IntListNode> &left, const std::vector<IntListNode> &right)
	{
		if constexpr (equal) {
			return std::equal(left.begin(), left.end(), right.begin(), right.end());
		} else {
			return std::lexicographical_compare(left.begin(), left.end(), right.begin(), right.end());
		}
	}

	bool operator()(int left, const std::vector<IntListNode> &right)
	{
		return (*this)(std::vector<IntListNode>(1, left), right);
	}

	bool operator()(const std::vector<IntListNode> &left, int right)
	{
		return (*this)(left, std::vector<IntListNode>(1, right));
	}
};

bool operator<(const IntListNode &left, const IntListNode &right)
{
	return std::visit(IntListNode_Comparison_Visitor<false>{}, left, right);
}

bool operator==(const IntListNode &left, const IntListNode &right)
{
	return std::visit(IntListNode_Comparison_Visitor<true>{}, left, right);
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

//	std::string example_text =
//		"[1,1,3,1,1]\n"
//		"[1,1,[5],1,1]\n"
//		"[[1],[2,3,4]]\n"
//		"[[1],4]\n"
//		"\n"
//		"[9]\n"
//		"[[8,7,6]]\n"
//		"\n"
//		"[[4,4],4,4]\n"
//		"[[4,4],4,4,4]\n"
//		"\n"
//		"[7,7,7,7]\n"
//		"[7,7,7]\n"
//		"\n"
//		"[]\n"
//		"[3]\n"
//		"\n"
//		"[[[]]]\n"
//		"[[]]\n"
//		"\n"
//		"[1,[2,[3,[4,[5,6,7]]]],8,9]\n"
//		"[1,[2,[3,[4,[5,6,0]]]],8,9]\n"
//	;
//	std::stringstream ss(example_text);

	int ordered_pair_index_sum = 0;
	std::vector<IntListNode> all_nodes;

	int pair_index = 1;
	IntListNode pd1, pd2;
	while (input >> pd1 >> pd2) {
		all_nodes.push_back(pd1);
		all_nodes.push_back(pd2);
		if (pd1 < pd2) ordered_pair_index_sum += pair_index;
		pair_index++;
	}

	std::cout << "Ordered pair index sum: " << ordered_pair_index_sum << '\n';

	std::string divider_packets = "[[2]] [[6]]"; // this is genuinely the easiest way of constructing this godawful datastructure
	std::stringstream divider_stream(divider_packets);
	divider_stream >> pd1 >> pd2;
	all_nodes.push_back(pd1);
	all_nodes.push_back(pd2);
	std::sort(all_nodes.begin(), all_nodes.end());

	int divkey1 = 0;
	int divkey2 = 0;
	for (size_t idx = 0; idx < all_nodes.size(); idx++) {
		const auto &elem = all_nodes[idx];

		if (divkey1 == 0 && elem == pd1) divkey1 = idx + 1;
		if (elem == pd2) {
			divkey2 = idx + 1;
			// definitely comes after divkey1, so stop here
			break;
		}
	}
	std::cout << "Decoder key: " << divkey1 * divkey2 << '\n';
}
