import std.container : RedBlackTree;
import std.format : formattedRead;
import std.file : readText;
import std.stdio : writeln;
import std.range : dropOne;
import std.typecons : Tuple;
import std.string : splitLines;

alias Instruction = Tuple!(string, "opcode", int, "a", int, "b", int, "c");

void main(string[] args)
{
	auto input = readText(args[1]).splitLines;
	Instruction[] program;
	int ins_reg;
	input[0].formattedRead("#ip %d", &ins_reg);
	foreach (line; input.dropOne) {
		Instruction i;
		line.formattedRead("%s %d %d %d", &i.opcode, &i.a, &i.b, &i.c);
		program ~= i;
	}

	auto termination_values = new RedBlackTree!int;
	int r1, r4;
	int last;
	while (true) {
		r4 = r1 | 65536;
		r1 = program[7].a;
		while (r4 > 0) {
			r1 = (((r1 + (r4 & 255)) & program[10].b) * program[11].b) & program[12].b;
			r4 /= 256;
		}
		if (termination_values.empty) {
			writeln("Quickest to halt: ", r1); // p1
		}
		if (r1 in termination_values) {
			break;
		}
		termination_values.insert(r1);
		last = r1;
	}
	writeln("Longest to halt: ", last); // p2
}
