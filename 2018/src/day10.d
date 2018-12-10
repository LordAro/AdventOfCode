import std.array : array;
import std.algorithm : minElement, maxElement, map, canFind;
import std.conv : to;
import std.file : slurp;
import std.stdio : write, writeln;
import std.math : abs;
import std.typecons : Tuple, tuple;
import std.string : strip;

auto get_minmax(Tuple!(int, int)[] grid)
{
	auto xs = grid.map!(a => a[0]);
	auto ys = grid.map!(a => a[1]);
	return tuple!("min_x", "max_x", "min_y", "max_y")(xs.minElement, xs.maxElement,
	                                                  ys.minElement, ys.maxElement);
}

void print_grid(Tuple!(int, int)[] grid)
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

ulong get_gridsize(Tuple!(int, int)[] grid)
{
	auto minmax = get_minmax(grid);
	return cast(ulong)abs(minmax.max_x - minmax.min_x) * cast(ulong)abs(minmax.max_y - minmax.min_y);
}

void main(string[] args)
{
	auto input = slurp!(string, string, string, string)(args[1], "position=<%s, %s> velocity=<%s, %s>")
		.map!(t => tuple(tuple(to!int(t[0].strip), to!int(t[1].strip)),
		                 tuple(to!int(t[2].strip), to!int(t[3].strip))));
	Tuple!(int, int)[] points = input.map!(t => t[0]).array;
	Tuple!(int, int)[] velos = input.map!(t => t[1]).array;

	auto grid_size = get_gridsize(points);

	int s = 1;
	for (; s < 12000; s++) {
		for (size_t i = 0; i < points.length; i++) {
			points[i][0] += velos[i][0];
			points[i][1] += velos[i][1];
		}
		auto new_grid_size = get_gridsize(points);
		if (new_grid_size > grid_size) {
			// Rewind a timestep
			for (size_t i = 0; i < points.length; i++) {
				points[i][0] -= velos[i][0];
				points[i][1] -= velos[i][1];
			}
			s--;
			break;
		}
		grid_size = new_grid_size;
	}
	writeln("Grid:");
	print_grid(points);
	writeln("At time ", s, "s");
}
