import std.algorithm;
import std.file;
import std.stdio;
import std.range;

void main(string[] args)
{
	auto lines = slurp!(string)(args[1], "%s");
	int twos = 0;
	int threes = 0;
	string common_letters;
	foreach (i, id; lines) {
		int[char] chars;
		foreach (c; id) {
			if (c in chars) {
				chars[c]++;
			} else {
				chars[c] = 1;
			}
		}
		foreach (v; chars) {
			if (v == 2) {
				twos++;
				break;
			}
		}
		foreach (v; chars) {
			if (v == 3) {
				threes++;
				break;
			}
		}

		foreach (id2; takeExactly(lines, i)) {
			auto p = levenshteinDistanceAndPath(id, id2);
			if (p[0] == 1) {
				auto idx = p[1].countUntil('s');
				common_letters = id.dup.remove(idx);
			}
		}
	}
	writeln("Checksum: ", twos * threes);
	writeln("Common letters: ", common_letters);
}
