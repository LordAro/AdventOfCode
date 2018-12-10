import std.array : array;
import std.algorithm : minElement, maxElement, map, canFind, min, max;
import std.conv : to;
import std.file : slurp;
import std.stdio : write, writeln;
import std.math : abs;
import std.range : iota;
import std.typecons : Tuple, tuple;
import std.string : strip;

auto get_minmax(G)(in auto ref G grid)
{
	auto xs = grid.map!(a => a[0]);
	auto ys = grid.map!(a => a[1]);
	return tuple!("min_x", "max_x", "min_y", "max_y")(xs.minElement, xs.maxElement,
	                                                  ys.minElement, ys.maxElement);
}

void print_grid(G)(in auto ref G grid)
{
	auto minmax = get_minmax(grid);
	for (int j = minmax.min_y; j <= minmax.max_y; j++) {
		for (int i = minmax.min_x; i <= minmax.max_x; i++) {
			if (grid.canFind!(a => a == tuple(i, j))) {
				write('#');
			} else {
				write('.');
			}
		}
		writeln();
	}
}

ulong get_gridsize(G)(in auto ref G grid)
{
	auto minmax = get_minmax(grid);
	return cast(ulong)abs(minmax.max_x - minmax.min_x) * cast(ulong)abs(minmax.max_y - minmax.min_y);
}

// Without having to get the grid first
ulong get_gridsize(G)(in auto ref G points, in auto ref G velocities, int factor)
{
	int min_x = int.max;
	int max_x = int.min;
	int min_y = int.max;
	int max_y = int.min;
	foreach (i; points.length.iota) {
		auto x = points[i][0] + (factor * velocities[i][0]);
		auto y = points[i][1] + (factor * velocities[i][1]);
		min_x = min(min_x, x);
		max_x = max(max_x, x);
		min_y = min(min_y, y);
		max_y = max(max_y, y);
	}
	return cast(ulong)abs(max_x - min_x) * cast(ulong)abs(max_y - min_y);
}

G get_grid(G)(in auto ref G points, in auto ref G velocities, int factor)
{
	return points.length.iota.map!(i => tuple(points[i][0] + (factor * velocities[i][0]), points[i][1] + (factor * velocities[i][1]))).array;
}

void main(string[] args)
{
	auto input = slurp!(string, string, string, string)(args[1], "position=<%s, %s> velocity=<%s, %s>")
		.map!(t => tuple(tuple(to!int(t[0].strip), to!int(t[1].strip)),
		                 tuple(to!int(t[2].strip), to!int(t[3].strip))));
	immutable auto points = input.map!(t => t[0]).array; // Both need to be arrays so it doesn't try to recompute values each time
	immutable auto velos = input.map!(t => t[1]).array;

	int l = 0;
	int u = 1;
	while (true) {
		int u2 = 1;
		for (;; u2 *= 2) {
			auto s1 = get_gridsize(points, velos, l + u2);
			auto s2 = get_gridsize(points, velos, l + u2 + 1);
			if (s1 < s2) break;
		}
		u = l + u2;
		l = l + (u2 / 2);
		if (u2 == 1) break; // solution in u
	}
	int s = 1;
	writeln("Grid:");
	print_grid(get_grid(points, velos, u));
	writeln("At time ", u, "s");
}
