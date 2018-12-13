import std.algorithm : canFind, strip, map, sum, filter;
import std.range : enumerate, takeOne, drop, split, back, array;
import std.stdio : writeln, File;

long index_sum(bool[] a, long start_idx)
{
	return a.enumerate.map!(b => b[1] ? cast(long)b[0] + start_idx : 0).sum;
}

void main(string[] args)
{
	auto input = File(args[1]).byLine;
	bool[] initial_state = input.takeOne.front.split.back.map!(s => s == '#').array;

	bool[][] transforms = input
		.drop(2)
		.map!(s => s.split(" => "))
		.filter!(t => t[1][0] == '#')              // Only store the patterns that result in a #
		.map!(t => t[0].map!(c => c == '#').array) // Convert pattern to bool array, where # is true
		.array;

	long start_idx = 0;
	for (int s = 0; s < 300; s++) {
		// Make sure we've got a head/tail that's long enough to match all patterns
		while (initial_state[0 .. 4].canFind(true)) {
			initial_state = false ~ initial_state;
			start_idx -= 1;
		}
		while (initial_state[$ - 4 .. $].canFind(true)) {
			initial_state ~= false;
		}

		bool[] new_state;
		new_state.reserve(initial_state.length);
		// No .slide until 2.079 :(. The below seems to be faster anyway...
		for (size_t i = 0; i < initial_state.length - 5; i++) {
			bool[] c = initial_state[i .. i + 5];
			new_state ~= transforms.canFind(c);
		}

		if (initial_state.strip!(b => !b) == new_state.strip!(b => !b)) {
			// Reached a stable point, bail
			start_idx += 50_000_000_000 - s;
			break;
		}

		start_idx += 2;
		initial_state = new_state;
		if (s + 1 == 20) writeln("Index sum at s=20: ", initial_state.index_sum(start_idx));
	}
	writeln("Index sum at s=50_000_000_000: ", initial_state.index_sum(start_idx));
}
