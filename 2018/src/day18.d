import std.algorithm : filter, each, map, count, joiner, find;
import std.file : readText;
import std.range : array, empty;
import std.stdio : writeln;
import std.string : splitLines;
import std.typecons : Tuple, tuple;

alias Coord = Tuple!(ulong, "x", ulong, "y");

auto adjacents(Coord c, Coord exmax)
{
	return [
		Coord(c.x - 1, c.y - 1), Coord(c.x, c.y - 1), Coord(c.x + 1, c.y - 1),
		Coord(c.x - 1, c.y),                          Coord(c.x + 1, c.y),
		Coord(c.x - 1, c.y + 1), Coord(c.x, c.y + 1), Coord(c.x + 1, c.y + 1)
	].filter!(d => d.x < exmax.x && d.y < exmax.y);
}

void main(string[] args)
{
	char[][] grid = readText(args[1]).splitLines.map!(r => r.dup).array;
	Coord grid_size = Coord(grid[0].length, grid.length);

	char[][][] grids = [grid];

	ulong cycle_idx = 0;
	int s = 0;
	for (; s < 1_000_000_000; s++) {
		char[][] new_grid = new char[][](grid.length, grid[0].length);
		for (ulong j = 0; j < grid.length; j++) {
			for (ulong i = 0; i < grid[j].length; i++) {
				auto adj = adjacents(Coord(i, j), grid_size).map!(c => grid[c.y][c.x]);
				auto adj_trees = adj.count!(e => e == '|');
				auto adj_lumber = adj.count!(e => e == '#');
				final switch (grid[j][i]) {
					case '.':
						new_grid[j][i] = adj_trees >= 3 ? '|' : '.';
						break;
					case '|':
						new_grid[j][i] = adj_lumber >= 3 ? '#' : '|';
						break;
					case '#':
						new_grid[j][i] = (adj_trees >= 1 && adj_lumber >= 1) ? '#' : '.';
						break;
				}
			}
		}

		auto cycle = grids.find(new_grid);
		if (!cycle.empty) {
			cycle_idx = grids.length - cycle.length;
			break;
		}
		grids ~= new_grid;
		grid = new_grid;
		if (s == 9) {
			auto trees = grid.joiner.count!(e => e == '|');
			auto lumber = grid.joiner.count!(e => e == '#');
			writeln("Resource value: ", trees * lumber);
		}
	}
	for (ulong g = cycle_idx; g < s; g++) {
		writeln(g);
	auto trees = grids[g].joiner.count!(e => e == '|');
	auto lumber = grids[g].joiner.count!(e => e == '#');
	writeln("Resource value: ", trees * lumber);
	}
}
