import std.algorithm : map, minElement, maxElement, canFind, sum;
import std.file : slurp;
import std.math : abs;
import std.range : retro;
import std.stdio : writeln;
import std.typecons : tuple, Tuple;

int distance(Tuple!(int, int) a, Tuple!(int, int) b)
{
	return abs(b[0] - a[0]) + abs(b[1] - a[1]);
}

Tuple!(int, int) get_closest(Range)(Range points, Tuple!(int, int) point)
{
	return points.minElement!(a => distance(a, point));
}

void main(string[] args)
{
	auto points = slurp!(int, int)(args[1], "%d, %d");

	int max_x = points.map!(a => a[0]).maxElement;
	int min_x = points.map!(a => a[0]).minElement;
	int max_y = points.map!(a => a[1]).maxElement;
	int min_y = points.map!(a => a[1]).minElement;

	int[Tuple!(int, int)] closest_grid;
	Tuple!(int, int)[] inf_points;
	int distance_region_size;
	for (int i = min_x - 1; i <= max_x + 1; i++) {
		for (int j = min_y - 1; j <= max_y + 1; j++) {
			auto cur_pos = tuple(i, j);
			auto closest = points.get_closest(cur_pos);
			if (i == min_x - 1 || i == max_x + 1 || j == min_y - 1 || j == max_y + 1) {
				inf_points ~= closest;
				closest_grid.remove(closest);
			} else if (!inf_points.canFind(closest) && closest == points.retro.get_closest(cur_pos)) {
				closest_grid[closest]++;
			}
			if (points.map!(a => distance(cur_pos, a)).sum < 10000) {
				distance_region_size++;
			}
		}
	}
	writeln("Maximum size: ", closest_grid.values.maxElement);
	writeln("Distance region size: ", distance_region_size);
}
