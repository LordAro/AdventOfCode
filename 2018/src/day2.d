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
		bool hasTwo = false;
		bool hasThree = false;
		foreach (v; chars) {
			if (!hasTwo && v == 2) {
				twos++;
				hasTwo = true;
			} else if (!hasThree && v == 3) {
				threes++;
				hasThree = true;
			}
		}

		// Part 2
		foreach (id2; drop(lines, i)) {
			int diff = 0;
			size_t diff_idx = 0;
			foreach (j, c; id) {
				if (c != id2[j]) {
					diff++;
					diff_idx = j;
				}
				if (diff > 1) {
					break;
				}
			}
			if (diff == 1) {
				common_letters = cast(string)((cast(ubyte[])id.dup).remove(diff_idx));
				break;
			}
		}
	}
	writeln("Checksum: ", twos * threes);
	writeln("Common letters: ", common_letters);
}
