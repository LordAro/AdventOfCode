import std.algorithm;
import std.format : formattedRead;
import std.math : sqrt;
import std.stdio : writeln, File, readln;
import std.range : back, front, takeOne, dropOne;
import std.typecons : Tuple;

int[6] addr(int[6] regs, int a, int b, int c)
{
	regs[c] = regs[a] + regs[b];
	return regs;
}

int[6] addi(int[6] regs, int a, int b, int c)
{
	regs[c] = regs[a] + b;
	return regs;
}

int[6] mulr(int[6] regs, int a, int b, int c)
{
	regs[c] = regs[a] * regs[b];
	return regs;
}

int[6] muli(int[6] regs, int a, int b, int c)
{
	regs[c] = regs[a] * b;
	return regs;
}

int[6] banr(int[6] regs, int a, int b, int c)
{
	regs[c] = regs[a] & regs[b];
	return regs;
}

int[6] bani(int[6] regs, int a, int b, int c)
{
	regs[c] = regs[a] & b;
	return regs;
}

int[6] borr(int[6] regs, int a, int b, int c)
{
	regs[c] = regs[a] | regs[b];
	return regs;
}

int[6] bori(int[6] regs, int a, int b, int c)
{
	regs[c] = regs[a] | b;
	return regs;
}

int[6] setr(int[6] regs, int a, int b, int c)
{
	regs[c] = regs[a];
	return regs;
}

int[6] seti(int[6] regs, int a, int b, int c)
{
	regs[c] = a;
	return regs;
}

int[6] gtir(int[6] regs, int a, int b, int c)
{
	regs[c] = a > regs[b] ? 1 : 0;
	return regs;
}

int[6] gtri(int[6] regs, int a, int b, int c)
{
	regs[c] = regs[a] > b ? 1 : 0;
	return regs;
}

int[6] gtrr(int[6] regs, int a, int b, int c)
{
	regs[c] = regs[a] > regs[b] ? 1 : 0;
	return regs;
}

int[6] eqir(int[6] regs, int a, int b, int c)
{
	regs[c] = a == regs[b] ? 1 : 0;
	return regs;
}

int[6] eqri(int[6] regs, int a, int b, int c)
{
	regs[c] = regs[a] == b ? 1 : 0;
	return regs;
}

int[6] eqrr(int[6] regs, int a, int b, int c)
{
	regs[c] = regs[a] == regs[b] ? 1 : 0;
	return regs;
}

alias instruction_func = int[6] function(int[6], int, int, int);

alias Instruction = Tuple!(string, "opcode", int, "a", int, "b", int, "c");

void main(string[] args)
{
	// For some reason, this cannot be placed in global/static scope
	immutable instruction_func[string] INS_MAP = [
		"addr": &addr,
		"addi": &addi,
		"mulr": &mulr,
		"muli": &muli,
		"banr": &banr,
		"bani": &bani,
		"borr": &borr,
		"bori": &bori,
		"setr": &setr,
		"seti": &seti,
		"gtir": &gtir,
		"gtri": &gtri,
		"gtrr": &gtrr,
		"eqir": &eqir,
		"eqri": &eqri,
		"eqrr": &eqrr,
	];

	auto input = File(args[1]).byLine;
	Instruction[] program;
	int ins_reg;
	input.takeOne.front.formattedRead("#ip %d", &ins_reg);
	foreach (line; input.dropOne) {
		Instruction i;
		line.formattedRead("%s %d %d %d", &i.opcode, &i.a, &i.b, &i.c);
		program ~= i;
	}

	int[] termination_values;
	int[6] registers;
	auto ipointer = &registers[ins_reg];

	while (*ipointer < program.length) {
		auto ins = program[*ipointer];
		if (ins == Instruction("eqrr", 5, 0, 3)) {
			if (termination_values.canFind(registers[5])) {
				break;
			}
			termination_values ~= registers[5];
		}
		registers = INS_MAP[ins.opcode](registers, ins.a, ins.b, ins.c);
		(*ipointer)++;
	}
	writeln("Quickest to halt: ", termination_values.front);
	writeln("Longest to halt: ", termination_values.back);
}
