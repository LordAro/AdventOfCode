import std.range;
import std.algorithm;
import std.container;
import std.file;
import std.format;
import std.stdio;
import std.string;
import std.typecons;

enum Ins {
	addr,
	addi,
	mulr,
	muli,
	banr,
	bani,
	borr,
	bori,
	setr,
	seti,
	gtir,
	gtri,
	gtrr,
	eqir,
	eqri,
	eqrr,
}

int[4] addr(int[4] regs, int a, int b, int c)
{
	regs[c] = regs[a] + regs[b];
	return regs;
}

int[4] addi(int[4] regs, int a, int b, int c)
{
	regs[c] = regs[a] + b;
	return regs;
}

int[4] mulr(int[4] regs, int a, int b, int c)
{
	regs[c] = regs[a] * regs[b];
	return regs;
}

int[4] muli(int[4] regs, int a, int b, int c)
{
	regs[c] = regs[a] * b;
	return regs;
}

int[4] banr(int[4] regs, int a, int b, int c)
{
	regs[c] = regs[a] & regs[b];
	return regs;
}

int[4] bani(int[4] regs, int a, int b, int c)
{
	regs[c] = regs[a] & b;
	return regs;
}

int[4] borr(int[4] regs, int a, int b, int c)
{
	regs[c] = regs[a] | regs[b];
	return regs;
}

int[4] bori(int[4] regs, int a, int b, int c)
{
	regs[c] = regs[a] | b;
	return regs;
}

int[4] setr(int[4] regs, int a, int b, int c)
{
	regs[c] = regs[a];
	return regs;
}

int[4] seti(int[4] regs, int a, int b, int c)
{
	regs[c] = a;
	return regs;
}

int[4] gtir(int[4] regs, int a, int b, int c)
{
	regs[c] = a > regs[b] ? 1 : 0;
	return regs;
}

int[4] gtri(int[4] regs, int a, int b, int c)
{
	regs[c] = regs[a] > b ? 1 : 0;
	return regs;
}

int[4] gtrr(int[4] regs, int a, int b, int c)
{
	regs[c] = regs[a] > regs[b] ? 1 : 0;
	return regs;
}

int[4] eqir(int[4] regs, int a, int b, int c)
{
	regs[c] = a == regs[b] ? 1 : 0;
	return regs;
}

int[4] eqri(int[4] regs, int a, int b, int c)
{
	regs[c] = regs[a] == b ? 1 : 0;
	return regs;
}

int[4] eqrr(int[4] regs, int a, int b, int c)
{
	regs[c] = regs[a] == regs[b] ? 1 : 0;
	return regs;
}

alias instruction_func = int[4] function(int[4], int, int, int);

void main(string[] args)
{
	// For some reason, this cannot be placed in global/static scope
	immutable instruction_func[Ins] INS_MAP = [
		Ins.addr: &addr,
		Ins.addi: &addi,
		Ins.mulr: &mulr,
		Ins.muli: &muli,
		Ins.banr: &banr,
		Ins.bani: &bani,
		Ins.borr: &borr,
		Ins.bori: &bori,
		Ins.setr: &setr,
		Ins.seti: &seti,
		Ins.gtir: &gtir,
		Ins.gtri: &gtri,
		Ins.gtrr: &gtrr,
		Ins.eqir: &eqir,
		Ins.eqri: &eqri,
		Ins.eqrr: &eqrr,
	];

	auto input = File(args[1]);

	Tuple!(int[4], int[4], int[4])[] mappings;
	for (string line = input.readln().strip(); !line.empty; line = input.readln().strip()) {
		int a, b, c, d;
		line.formattedRead("Before: [%d, %d, %d, %d]", &a, &b, &c, &d);
		int[4] before = [a, b, c, d];
		string ins_str = input.readln().strip();
		ins_str.formattedRead("%d %d %d %d", &a, &b, &c, &d);
		int[4] ins = [a, b, c, d];
		string after_str = input.readln().strip();
		after_str.formattedRead("After:  [%d, %d, %d, %d]", &a, &b, &c, &d);
		int[4] after = [a, b, c, d];
		mappings ~= tuple(ins, before, after);
		input.readln();
	}

	input.readln(); // Final blank line
	int[4][] program;
	string line;
    while ((line = input.readln()) !is null) {
		int a, b, c, d;
		line.formattedRead("%d %d %d %d", &a, &b, &c, &d);
		program ~= [a, b, c, d];
	}

	int multiple_ins_count = 0;
	Ins[][int] possible_ins;
	foreach (change; mappings) {

		auto ins = change[0];
		auto before = change[1];
		auto after = change[2];

		int possible_instructions = 0;
		foreach (i, func; INS_MAP) {
			if (func(before, ins[1], ins[2], ins[3]) == after) {
				possible_ins[ins[0]] ~= i;
				possible_instructions++;
			}
		}

		if (possible_instructions >= 3) multiple_ins_count++;
	}

	writeln("Number of instructions that could be multiple opcodes: ", multiple_ins_count);

	Ins[int] actual_ins;
	possible_ins = possible_ins.byKeyValue.map!(kv => tuple(kv.key, kv.value.sort.uniq.array)).assocArray;
	while (possible_ins.length != 0) {
		auto next = possible_ins.byKeyValue.find!(l => l.value.length == 1);
		if (next.empty) {
			writeln("Error, no remaining options");
			break;
		}
		auto ins = next.front;
		actual_ins[ins.key] = ins.value.front;
		foreach (i, l; possible_ins) {
			possible_ins[i] = l.filter!(a => a != actual_ins[ins.key]).array;
		}
		possible_ins.remove(ins.key);
	}

	int[4] registers = [0, 0, 0, 0];
	foreach(i; program) {
		registers = INS_MAP[actual_ins[i[0]]](registers, i[1], i[2], i[3]);
	}
	writeln("Value in register 0: ", registers[0]);
}
