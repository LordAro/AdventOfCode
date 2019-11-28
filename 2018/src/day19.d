import std.algorithm;
import std.format : formattedRead;
import std.math : sqrt;
import std.stdio : writeln, File;
import std.range : takeOne, dropOne;
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
	string first_line = input.takeOne.front.dup;
	first_line.formattedRead("#ip %d", &ins_reg);
	foreach (line; input.dropOne) {
		Instruction i;
		line.formattedRead("%s %d %d %d", &i.opcode, &i.a, &i.b, &i.c);
		program ~= i;
	}

	int[6] registers;
	auto ipointer = &registers[ins_reg];

	while (*ipointer < program.length) {
		auto ins = program[*ipointer];
		registers = INS_MAP[ins.opcode](registers, ins.a, ins.b, ins.c);
		(*ipointer)++;
	}

	writeln("Contents of register 0: ", registers[0]);

	// Ultimate goal of algorithm is sum of divisors of registers[3] once the main inner loop is set
	int[6] registers2;
	registers2[0] = 1;
	ipointer = &registers2[ins_reg];
	// Assumes nested loop always starts at position 1
	while (*ipointer < program.length && *ipointer != 1) {
		auto ins = program[*ipointer];
		registers2 = INS_MAP[ins.opcode](registers2, ins.a, ins.b, ins.c);
		(*ipointer)++;
	}

	writeln(registers2);
	auto largest = maxElement(registers2[]);
	int sum = largest + 1; // Base case
	for (int i = 2; i < cast(int)sqrt(cast(double)largest); i++) {
		if (largest % i == 0) {
			sum += i + (largest / i);
		}
	}
	writeln("Sum of divisors: ", sum);
}
