import std.algorithm;
import std.file;
import std.stdio;

void main(string[] args)
{
	auto lines = slurp!(int)(args[1], "%d");
	writeln("Resulting frequency: ", sum(lines));

	int[] seen = [];
	int cur_freq = 0;
	outer: while (true) {
		foreach (freq; lines) {
			if (!seen.canFind(cur_freq)) {
				seen ~= cur_freq;
			} else {
				break outer;
			}
			cur_freq += freq;
		}
	}
	writeln("First duplicate frequency: ", cur_freq);
}
