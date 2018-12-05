import std.ascii;
import std.algorithm : min, remove;
import std.file;
import std.stdio;
import std.string;

// TODO: Use linked list
ubyte[] collapse(ubyte[] polymer)
{
	for (size_t i = 1; i < polymer.length; i++) {
		auto c1 = polymer[i - 1];
		auto c2 = polymer[i];
		if ((isUpper(c1) && isLower(c2) && std.ascii.toLower(c1) == c2)
				|| (isLower(c1) && isUpper(c2) && std.ascii.toUpper(c1) == c2)) {
			replaceInPlace(polymer, i-1, i+1, cast(ubyte[])[]);
			i -= 2;
		}
	}
	return polymer;
}

void main(string[] args)
{
	auto input = readText(args[1]).strip();

	writeln("Final length: ", collapse(cast(ubyte[])input.dup).length);
	size_t min_len = 50000;
	for (byte c = 'a'; c <= 'z'; c++) {
		auto reduced = remove!(a => a == c || a == std.ascii.toUpper(c))(cast(ubyte[])input.dup);
		min_len = min(min_len, collapse(reduced).length);
	}
	writeln("Minimal length: ", min_len);
}
