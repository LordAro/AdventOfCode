import std.algorithm;
import std.conv;
import std.file;
import std.range;
import std.stdio;

R sum_metadata(R)(R r, ref int count)
{
	int num_child = r.front;
	r.popFront;
	int num_metadata = r.front;
	r.popFront;

	for (int i; i < num_child; i++) {
		r = r.sum_metadata(count);
	}

	count += r.take(num_metadata).sum;
	return r.dropExactly(num_metadata);
}

R node_value(R)(R r, ref int count)
{
	int num_child = r.front;
	r.popFront;
	int num_metadata = r.front;
	r.popFront;

	int[] child_values;
	for (int i; i < num_child; i++) {
		int val;
		r = r.node_value(val);
		child_values ~= val;
	}

	if (num_child == 0) {
		count += r.take(num_metadata).sum;
	} else {
		// Ignore out of range
		count += r.take(num_metadata).filter!(a => 0 < a && a <= child_values.length).map!(a => child_values[a - 1]).sum;
	}
	return r.drop(num_metadata);
}

void main(string[] args)
{
	auto input = readText(args[1]).split.map!(a => to!int(a));
	int sum;
	input.sum_metadata(sum);
	writeln("Metadata total: ", sum);
	int val;
	input.node_value(val);
	writeln("Node value: ", val);
}

unittest
{
	auto input = [2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2];
	int sum;
	input.sum_metadata(sum);
	assert(sum == 138);
}

unittest
{
	auto input = [2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2];
	int val;
	input.node_value(val);
	assert(val == 66);
}
