import std.algorithm : canFind, strip, map, sum;
import std.range : enumerate, takeOne, drop, split, back, array, assocArray;
import std.stdio : writeln, File;
import std.typecons : tuple;

long index_sum(bool[] a, long start_idx)
{
	return a.enumerate.map!(b => b[1] ? cast(long)b[0] + start_idx : 0).sum;
}

void main(string[] args)
{
	auto input = File(args[1]).byLine;
	bool[] input_state = input.takeOne.front.split.back.map!(s => s == '#').array;

	bool[bool[]] transforms = input
		.drop(2)
		.map!(s => s.split(" => "))
		.map!(t => tuple(cast(const)t[0].map!(c => c == '#').array, t[1][0] == '#'))
		.assocArray;

	auto old_state = input_state.dup;
	long start_idx = 0;
	for (int s = 0; s < 300; s++) {
		while (old_state[0 .. 5].canFind(true)) {
			old_state = false ~ old_state;
			start_idx -= 1;
		}
		while (old_state[$ - 5 .. $].canFind(true)) {
			old_state ~= false;
		}

		bool[] new_state;
		for (size_t i = 0; i < old_state.length - 5; i++) {
			bool[] c = old_state[i .. i + 5];
			if (c in transforms) {
				new_state ~= transforms[c];
			} else {
				new_state ~= false;
			}
		}

		if (old_state.strip!(b => !b) == new_state.strip!(b => !b)) {
			// Reached a stable point, bail
			start_idx += 50_000_000_000 - s;
			break;
		}

		start_idx += 2;
		old_state = new_state;
		if (s + 1 == 20) writeln("Index sum at s=20: ", old_state.index_sum(start_idx));
	}
	writeln("Index sum at s=50_000_000_000: ", old_state.index_sum(start_idx));
}
