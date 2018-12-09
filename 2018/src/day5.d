import std.ascii : toUpper;
import std.algorithm : min;
import std.file : readText;
import std.container : SList;
import std.range : dropBackOne;
import std.stdio : writeln;
import std.typecons : tuple, Tuple;

alias ByteStack = Tuple!(SList!ubyte, "s", size_t, "len");

ByteStack collapse(ByteStack polymer, size_t cur_min, ubyte ignore = 0xff)
{
	size_t len = 0;
	ByteStack reduced;
	ubyte ignore_upper = toUpper(ignore);
	size_t i = 0;
	foreach (c; polymer.s) {
		// Bail early if length has exceeded previous minimum - it can't get any smaller than this...
		auto remaining = polymer.len - i;
		if (reduced.len > remaining && reduced.len - remaining > cur_min) { // Remember to protect against underflows
			return tuple!("s", "len")(polymer.s, cur_min);
		}
		if (!reduced.s.empty && reduced.s.front != c && toUpper(reduced.s.front) == toUpper(c)) {
			reduced.s.removeFront();
			reduced.len--;
		} else if (ignore != c && ignore_upper != c) {
			reduced.s.insertFront(c);
			reduced.len++;
		}
		i++;
	}
	return reduced;
}

void main(string[] args)
{
	auto input = cast(ubyte[])readText(args[1]).dropBackOne;
	auto polymer = tuple!("s", "len")(SList!ubyte(input), input.length);

	auto reduced = collapse(polymer, input.length + 1);
	writeln("Final length: ", reduced[1]);

	size_t min_len = 50000;
	for (byte c = 'a'; c <= 'z'; c++) {
		min_len = collapse(reduced, min_len, c)[1]; // always returns minimum
	}
	writeln("Minimal length: ", min_len);
}
