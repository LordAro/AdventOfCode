import std.ascii : toUpper;
import std.algorithm : min, count;
import std.file : readText;
import std.container : SList;
import std.stdio : writeln;
import std.string : strip;

SList!ubyte collapse(SList!ubyte polymer, ubyte ignore = 0xff)
{
	SList!ubyte reduced;
	foreach (c; polymer) {
		if (!reduced.empty && reduced.front != c && toUpper(reduced.front) == toUpper(c)) {
			reduced.removeFront();
		} else if (ignore != c && toUpper(ignore) != c) {
			reduced.insertFront(c);
		}
	}
	return reduced;
}

void main(string[] args)
{
	auto polymer = SList!ubyte(cast(ubyte[])readText(args[1]).strip);

	auto reduced = collapse(polymer);
	writeln("Final length: ", reduced[].count);

	size_t min_len = 50000;
	for (byte c = 'a'; c <= 'z'; c++) {
		min_len = min(min_len, collapse(reduced, c)[].count);
	}
	writeln("Minimal length: ", min_len);
}

unittest
{
	assert(collapse("aA") == "");
	assert(collapse("abBA") == "");
	assert(collapse("abAB") == "abAB");
	assert(collapse("aabAAB") == "aabAAB");
}
