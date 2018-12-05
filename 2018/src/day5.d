import std.ascii : toUpper;
import std.algorithm : min;
import std.range : dropBackOne, retro;
import std.container : DList;
import std.file : read;
import std.stdio : writeln;
import std.string : count;

DList!ubyte collapse(DList!ubyte polymer)
{
	// Iterate backwards due to BidirectionalRange not actually being Bidirectional...
	auto reduced = DList!ubyte(polymer.back);
	foreach (c; polymer[].dropBackOne.retro) {
		auto b = reduced.front;
		if (b != c && toUpper(b) == toUpper(c)) {
			reduced.removeFront;
		} else {
			reduced.insertFront(c);
		}
	}
	return reduced;
}

DList!ubyte char_filter(DList!ubyte dll, ubyte r)
{
	DList!ubyte new_dll;
	ubyte r_u = cast(ubyte)toUpper(r);
	foreach (c; dll) {
		if (c != r && c != r_u) {
			new_dll.insertBack(c);
		}
	}
	return new_dll;
}

void main(string[] args)
{
	DList!ubyte polymer = DList!ubyte(cast(const(ubyte)[])read(args[1]));
	polymer.removeBack; // Newline

	DList!ubyte reduced = collapse(polymer);
	writeln("Final length: ", reduced[].count);

	size_t min_len = 50000;
	for (byte c = 'a'; c <= 'z'; c++) {
		DList!ubyte new_polymer = char_filter(reduced, c);
		min_len = min(min_len, collapse(new_polymer)[].count);
	}
	writeln("Minimal length: ", min_len);
}
