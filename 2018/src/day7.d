import std.array : empty, front;
import std.algorithm;
import std.file : slurp;
import std.range : array, enumerate;
import std.stdio : writeln;
import std.typecons : tuple, Tuple;

string process(ubyte[][ubyte] reqs, int worker_count, out int time)
{
	string output;
	time = 0;

	auto all_vals = reqs.values.joiner.uniq;
	ubyte[] to_search = reqs.keys.filter!(a => !all_vals.canFind(a)).array;
	auto workers = new Tuple!(char, int)[worker_count];

	while (!to_search.empty || workers.map!(a => a[1]).maxElement >= 0) {
		auto finished_workers = workers.filter!(a => a[1] == 0);
		foreach (w; finished_workers) {
			if (w[0] != char.init && w[1] == 0) {
				output ~= w[0];
				if (w[0] in reqs) {
					foreach (m; reqs[w[0]]) {
						auto other_keys = reqs.byKey.filter!(a => a != w[0]);
						if (!other_keys.map!(k => reqs[k]).joiner.uniq.canFind(m)) {
							to_search ~= m;
						}
					}
					reqs.remove(w[0]); // Remove edges
					sort(to_search); // Ensure alphabetical order
				}
			}
		}
		auto idle_workers = workers.filter!(a => a[1] <= 0);
		if (idle_workers.count == workers.length && to_search.empty) {
			break;
		}
		foreach (ref w; idle_workers) {
			if (!to_search.empty) {
				char n = to_search.front;
				to_search = to_search.remove(0);
				int cost = n - 'A' + 1 + 60;
				w = tuple(n, cost);
			}
		}
		time++;
		workers.each!((ref w) => w[1]--);
	}
	return output;
}

void main(string[] args)
{
	auto input = slurp!(char, char)(args[1], "Step %c must be finished before step %c can begin.");
	ubyte[][ubyte] reqs;
	foreach (req; input) {
		reqs[req[0]] ~= req[1];
	}

	int time = 42;
	string output = process(reqs.dup, 1, time);
	writeln("Instruction order: ", output);
	output = process(reqs.dup, 5, time);
	writeln("Instruction order with 5 workers: ", output, " ", time, "s");
}
