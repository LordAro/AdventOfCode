import std.algorithm;
import std.format : formattedRead;
import std.stdio : writeln, File;
import std.typecons;
import std.range;

alias Coord = Tuple!(ulong, "x", ulong, "y");

enum Dir {
	Left,
	Down,
	Right,
}

void fill_level(ref char[][] grid, Coord coord)
{
	auto left = Coord(coord.x - 1, coord.y);
	while (grid[left.y][left.x] == '|') {
		grid[left.y][left.x] = '~';
		left = Coord(left.x - 1, left.y);
	}
	while (grid[coord.y][coord.x] == '|') {
		grid[coord.y][coord.x] = '~';
		coord = Coord(coord.x + 1, coord.y);
	}
}

bool flow(ref char[][] grid, Coord coord, Dir dir)
{
	if (coord.y >= grid.length || coord.x >= grid[coord.y].length) {
		return true;
	}
	if (grid[coord.y][coord.x] == '#' || grid[coord.y][coord.x] == '~') {
		return false;
	}
	grid[coord.y][coord.x] = '|';

	bool leak = flow(grid, Coord(coord.x, coord.y + 1), Dir.Down);
	if (leak) return true;

	leak = false;
	if (dir != Dir.Left) {
		leak = flow(grid, Coord(coord.x + 1, coord.y), Dir.Right);
	}
	if (dir != Dir.Right) {
		leak |= flow(grid, Coord(coord.x - 1, coord.y), Dir.Left); // Bitwise, so don't shortcircuit
	}
	if (leak) return true;

	if (dir == Dir.Down) fill_level(grid, coord);

	return false;
}

void main(string[] args)
{
	Coord[] clay;
	foreach (l; File(args[1]).byLine) {
		char c1, c2;
		ulong n1, n2, n3;
		l.formattedRead("%c=%d, %c=%d..%d", &c1, &n1, &c2, &n2, &n3);
		if (c1 == 'x') {
			for (ulong j = n2; j <= n3; j++) {
				clay ~= Coord(n1, j);
			}
		} else {
			for (ulong i = n2; i <= n3; i++) {
				clay ~= Coord(i, n1);
			}
		}
	}
	auto ymin = clay.minElement!(c => c.y).y;
	auto ymax = clay.maxElement!(c => c.y).y;
	auto xmin = clay.minElement!(c => c.x).x - 1;
	auto xmax = clay.maxElement!(c => c.x).x + 1; // If the flow ends up in one of the lower corner boxes
	clay.each!((ref c) => c = Coord(c.x-xmin, c.y-ymin)); // Normalise

	auto grid = new char[][](ymax-ymin+1, xmax-xmin+1);
	foreach (ref r; grid) r.fill('.');
	foreach (c; clay) grid[c.y][c.x] = '#';

	flow(grid, Coord(500-xmin, 0), Dir.Down);

	auto stationary_water = grid.joiner.count!(c => c == '~');
	auto flowing_water = grid.joiner.count!(c => c == '|');
	writeln("Number of water tiles: ", stationary_water + flowing_water);
	writeln("Number of stationary water tiles: ", stationary_water);
}
