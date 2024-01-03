#include <cstdint>
#include <iostream>
#include <fstream>
#include <map>
#include <vector>

enum class Instruction {
	Hlf,
	Tpl,
	Inc,
	Jmp,
	Jie,
	Jio,
};

static const std::map<std::string, Instruction> INS_MAP = {
	{"hlf", Instruction::Hlf},
	{"tpl", Instruction::Tpl},
	{"inc", Instruction::Inc},
	{"jmp", Instruction::Jmp},
	{"jie", Instruction::Jie},
	{"jio", Instruction::Jio},
};

using program_t = std::vector<std::pair<Instruction, std::string>>;

void RunProgram(const program_t &program, uint32_t * const reg_a, uint32_t * const reg_b)
{
	uint32_t a = *reg_a;
	uint32_t b = *reg_b;

	for (size_t i = 0; i < program.size(); i++) {
		Instruction ins = program[i].first;
		std::string params = program[i].second;
		uint32_t *r;
		if (ins != Instruction::Jmp) {
			if (params[0] == 'a') {
				r = &a;
			} else if (params[0] == 'b') {
				r = &b;
			} else {
				std::cout << params << "\n";
				throw "Unknown register";
			}
		}
		switch (ins) {
			case Instruction::Hlf:
				*r /= 2;
				break;
			case Instruction::Tpl:
				*r *= 3;
				break;
			case Instruction::Inc:
				*r += 1;
				break;
			case Instruction::Jmp: {
				int offset = std::stoi(params);
				i += (offset-1); // i gets incremented by the for loop
				break;
			}
			case Instruction::Jie:
			case Instruction::Jio: {
				int spl = params.find(' ');
				int offset = std::stoi(params.substr(spl, params.size()-spl));
				if ((ins == Instruction::Jie && *r % 2 == 0) || (ins == Instruction::Jio && *r == 1)) {
					i += (offset-1);
				}
				break;
			}
		};
	}

	*reg_a = a;
	*reg_b = b;
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

	program_t program;

	std::string line;
	while (std::getline(input, line)) {
		size_t spl = line.find(' ');
		std::string ins_str = line.substr(0, spl);
		std::string params = line.substr(spl + 1, line.size() - spl - 1);

		program.emplace_back(INS_MAP.at(ins_str), params);
	}

	uint32_t a = 0;
	uint32_t b = 0;
	RunProgram(program, &a, &b);

	std::cout << "Value of b: " << b << "\n";

	a = 1;
	b = 0;
	RunProgram(program, &a, &b);

	std::cout << "Value of b (a==1): " << b << "\n";

	return 0;
}
