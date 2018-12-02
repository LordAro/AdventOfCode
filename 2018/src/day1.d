import std.algorithm;
import std.container.rbtree;
import std.file;
import std.stdio;

void main(string[] args)
{
	auto lines = slurp!(int)(args[1], "%d");
	writeln("Resulting frequency: ", sum(lines));

	auto seen = redBlackTree!int();
	int cur_freq = 0;
	outer: while (true) {
		foreach (freq; lines) {
			if (cur_freq in seen) {
				break outer;
			}
			seen.insert(cur_freq);
			cur_freq += freq;
		}
	}
	writeln("First duplicate frequency: ", cur_freq);
}