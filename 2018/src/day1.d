import std.algorithm;
import std.file;
import std.range;
import std.stdio;

void main(string[] args)
{
	auto lines = slurp!(int)(args[1], "%d");
	writeln("Resulting frequency: ", sum(lines));

	bool[int] seen;
	int cur_freq = 0;
	foreach (freq; cycle(lines)) {
		if (cur_freq in seen) {
			break;
		}
		seen[cur_freq] = true;
		cur_freq += freq;
	}
	writeln("First duplicate frequency: ", cur_freq);
}
