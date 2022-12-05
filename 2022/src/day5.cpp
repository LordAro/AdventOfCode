#include <cstdio>
#include <deque>
#include <fstream>
#include <iostream>
#include <vector>

struct MoveInstruction {
	int quantity;
	int from_idx;
	int to_idx;
};

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

	std::vector<std::deque<char>> stacks; // deques representing each stack. the back is the "top"
	std::vector<MoveInstruction> instrs;

	bool parsing_instructions = false;
	std::string line;
	while (std::getline(input, line)) {
		if (parsing_instructions) {
			MoveInstruction move_instr;
			int count = sscanf(line.c_str(), "move %d from %d to %d", &move_instr.quantity, &move_instr.from_idx, &move_instr.to_idx);
			/* while not EOF (blank lines at end may be in the loop, but the result of which should not be included in output..) */
			if (count == 3) {
				instrs.push_back(move_instr);
			}
		} else if (line.empty()) {
			parsing_instructions = true;
		} else {
			for (size_t i = 1; i < line.size(); i += 4) {
				if (std::isdigit(line[i])) break; // done
				size_t stack_no = (i - 1) / 4;
				while (stack_no >= stacks.size()) {
					stacks.emplace_back();
				}

				if (line[i] != ' ') {
					stacks[stack_no].push_front(line[i]);
				}
			}
		}
	}

//	stacks = { {'Z', 'N'}, {'M', 'C', 'D'}, {'P'} }; // example
//	instrs = { {1, 2, 1}, {3, 1, 3}, {2, 2, 1}, {1, 1, 2} };
	auto stacks9001 = stacks; // part2

	for (const auto &instr : instrs) {
		size_t from_idx = instr.from_idx - 1; // 1-based -> 0-based
		size_t to_idx = instr.to_idx - 1;
		std::vector<char> crane9001;
		for (int move = 0; move < instr.quantity; move++) {
			char item = stacks[from_idx].back();
			stacks[from_idx].pop_back();
			stacks[to_idx].push_back(item);

			crane9001.push_back(stacks9001[from_idx].back());
			stacks9001[from_idx].pop_back();
		}

		for (auto it = crane9001.rbegin(); it != crane9001.rend(); ++it) {
			stacks9001[to_idx].push_back(*it);
		}
	}

	std::cout << "Crate on top of each stack: ";
	for (const auto &stack : stacks) std::cout << stack.back();
	std::cout << '\n';

	std::cout << "Crate on top of each stack (after using CrateMover9001): ";
	for (const auto &stack : stacks9001) std::cout << stack.back();
	std::cout << '\n';
}
