import std.array;
import std.algorithm;
import std.conv;
import std.file;
import std.range;
import std.stdio;
import std.string;

int count_metadata(ref int[] r)
{
	auto header = r.take(2);
	int num_child = header.front;
	int num_metadata = header.back;
	r = r.drop(2);
	int count;
	for (int i; i < num_child; i++) {
		count += r.count_metadata;
	}
	count += r.take(num_metadata).sum;
	r = r.drop(num_metadata);
	return count;
}

int node_value(ref int[] r)
{
	auto header = r.take(2);
	int num_child = header.front;
	int num_metadata = header.back;
	r = r.drop(2);
	int[] child_values;
	for (int i; i < num_child; i++) {
		child_values ~= node_value(r);
	}
	int count;
	if (num_child == 0) {
		count = r.take(num_metadata).sum;
	} else {
		count = r.take(num_metadata).filter!(a => 0 < a && a <= child_values.length).map!(a => child_values[a - 1]).sum;
	}
	r = r.drop(num_metadata);
	return count;
}

void main(string[] args)
{
	auto input = readText(args[1]).split.map!(a => to!int(a)).array;
	//auto input = [2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2];
	auto p1 = input.dup;
	writeln("Metadata total: ", count_metadata(p1));
	writeln("Node value: ", node_value(input));
}
