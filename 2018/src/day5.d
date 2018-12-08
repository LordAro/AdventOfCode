import std.ascii : toUpper;
import std.algorithm : min, count;
import std.file : readText;
import std.container : SList;
import std.stdio : writeln;
import std.string : strip;
import std.typecons : tuple, Tuple;

Tuple!(SList!ubyte, size_t) collapse(SList!ubyte polymer, ubyte ignore = 0xff)
{
	size_t len;
	SList!ubyte reduced;
	foreach (c; polymer) {
		if (!reduced.empty && reduced.front != c && toUpper(reduced.front) == toUpper(c)) {
			reduced.removeFront();
			len--;
		} else if (ignore != c && toUpper(ignore) != c) {
			reduced.insertFront(c);
			len++;
		}
	}
	return tuple(reduced, len);
}

void main(string[] args)
{
	auto polymer = SList!ubyte(cast(ubyte[])readText(args[1]).strip);

	auto reduced = collapse(polymer);
	writeln("Final length: ", reduced[1]);

	size_t min_len = 50000;
	for (byte c = 'a'; c <= 'z'; c++) {
		min_len = min(min_len, collapse(reduced[0], c)[1]);
	}
	writeln("Minimal length: ", min_len);
}

unittest
{
	assert(collapse("aA") == tuple("", 0));
	assert(collapse("abBA") == tuple("", 0));
	assert(collapse("abAB") == tuple("abAB", 4));
	assert(collapse("aabAAB") == tuple("aabAAB", 6));
}
