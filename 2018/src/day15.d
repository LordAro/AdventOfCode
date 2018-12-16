import std.algorithm : canFind, count, filter, joiner, map, minElement, minPos, remove, reverse, sort, sum;
import std.file : readText;
import std.math : abs;
import std.range : array, back, empty, front;
import std.stdio : write, writeln;
import std.string : format, splitLines, strip;
import std.typecons : tuple, Tuple;

alias Coord = Tuple!(ulong, "x", ulong, "y");

struct Person {
	Coord pos;
	int health = 200;
	bool isElf;

	string toString()
	{
		return "%c@%d,%d %dhp".format(isElf ? 'E' : 'G', pos.x, pos.y, health);
	}
}

bool is_adjacent(Coord a, Coord b)
{
	return a == tuple(b.x + 1, b.y) || a == tuple(b.x - 1, b.y)
			|| a == tuple(b.x, b.y + 1) || a == tuple(b.x, b.y - 1);
}

auto blank_adjacents(char[][] grid, Coord a)
{
	return [Coord(a.x, a.y - 1), Coord(a.x - 1, a.y), Coord(a.x + 1, a.y), Coord(a.x, a.y + 1)]
		.filter!(c => c.x < grid[0].length && c.y < grid.length && grid[c.y][c.x] == '.');
}

// Debugging
void print_grid(char[][] grid, Person[] combatants)
{
	foreach (j, r; grid) {
		write(r);

		foreach (c; combatants.sort!((p, q) => p.pos.x < q.pos.x).alive.filter!(c => c.pos.y == j)) {
			write(' ', c.isElf ? 'E' : 'G', '(', c.health, ')');
		}
		writeln();
	}
}

alias alive = filter!(c => c.health > 0);
alias elves = filter!(c => c.isElf);
alias goblins = filter!(c => !c.isElf);

void move_combatant(ref char[][] grid, ref Person p, Coord new_pos)
{
	assert(grid[p.pos.y][p.pos.x] == (p.isElf ? 'E' : 'G'));
	assert(grid[new_pos.y][new_pos.x] == '.');
	assert(is_adjacent(new_pos, p.pos));

	grid[new_pos.y][new_pos.x] = (p.isElf ? 'E' : 'G');
	grid[p.pos.y][p.pos.x] = '.';
	p.pos = new_pos;
}

auto get_adjacent(Person p, Person[] combatants)
{
	return combatants.alive.filter!(c => is_adjacent(c.pos, p.pos)).filter!(c => c.isElf != p.isElf);
}

ulong manhattan(Coord a, Coord b)
{
	return abs(cast(int)a.x - cast(int)b.x) + abs(cast(int)a.y - cast(int)b.y);
}

Coord[] AStar(char[][] grid, Coord start, Coord end)
{
	Coord[] searched;
	Coord[] toSearch = [start];

	Coord[Coord] cameFrom;

	ulong[Coord] gScore;
	gScore[start] = 0;

	ulong[Coord] fScore;
	fScore[start] = manhattan(start, end);

	while(!toSearch.empty) {
		Coord current = toSearch.minElement!(n => n in fScore ? fScore[n] : ulong.max);
		if (current == end) {
			// Reconstruct path
			Coord[] total_path = [current];
			while (current in cameFrom) {
				current = cameFrom[current];
				total_path ~= current;
			}
			return total_path.reverse;
		}

		toSearch = toSearch.remove!(a => a == current);
		searched ~= current;

		foreach (n; grid.blank_adjacents(current)) {
			if (searched.canFind(n)) continue;

			ulong tentative = gScore[current] + 1; // always manhattan of 1
			if (!toSearch.canFind(n)) {
				toSearch ~= n;
			} else if (tentative >= (n in gScore ? gScore[n] : ulong.max)) {
				continue;
			}

			cameFrom[n] = current;
			gScore[n] = tentative;
			fScore[n] = gScore[n] + manhattan(n, end);
		}
	}

	return []; // No path :(
}

bool reading_order(Coord a, Coord b)
{
	return a.y < b.y || (a.y == b.y && a.x < b.x);
}

Tuple!(int, int) playGame(char[][] grid)
{
	Person[] combatants;

	// Find combatants
	foreach (j, r; grid) {
		foreach(i, c; r) {
			if (c == 'E' || c == 'G') {
				Person e = {pos: tuple(i, j), isElf: c == 'E'};
				combatants ~= e;
			}
		}
	}

	int round = 1;
	outer: for (; (combatants.elves.alive.count > 0 && combatants.goblins.alive.count > 0); round++) {
		// Make sure movements happen top to bottom, left to right
		combatants.sort!((a, b) => reading_order(a.pos, b.pos));

		foreach(ref p; combatants.alive) {
			if (p.health <= 0) continue; // Might have been killed earlier in the round

			if (combatants.alive.filter!(q => p.isElf != q.isElf).count == 0) break outer;

			// If not next to any target, move towards one
			auto adjacents = p.get_adjacent(combatants);
			if (adjacents.count == 0) {
				auto targets = combatants.alive.filter!(c => c.isElf != p.isElf).map!(c => c.pos);
				auto target_adjacents = targets.map!(c => grid.blank_adjacents(c)).joiner;
				auto source_adjacents = grid.blank_adjacents(p.pos);

				Coord[][] routes;
				foreach (s; source_adjacents) {
					foreach (t; target_adjacents) {
						auto route = AStar(grid, s, t);
						if (route.length != 0) {
							routes ~= route;
						}
					}
				}

				if (routes.count > 0) {
					// Choose shortest distance, or reading order, or reading order of first move
					alias nextMove = (m, n) => m.length < n.length
						|| (m.length == n.length && reading_order(m.back, n.back)
							|| (m.back == n.back && reading_order(m.front, n.front)));
					auto closest_target_moves = routes.minElement!(nextMove);

					auto next_position = closest_target_moves.front;
					move_combatant(grid, p, next_position);
				}
			}

			adjacents = p.get_adjacent(combatants);
			if (adjacents.count > 0) {
				// Attack adjacent with lowest health, or reading order if equal
				// Must use minPos to get reference
				auto target = adjacents.minPos!((p, q) => p.health < q.health || (p.health == q.health && reading_order(p.pos, q.pos)));
				target.front.health -= 3;
				if (target.front.health <= 0) {
					grid[target.front.pos.y][target.front.pos.x] = '.';
				}
			}
		}

		//writeln("After round ", round);
		//print_grid(grid, combatants);
		//writeln();
	}

	// tuple(round, remaining health) - round always ends on the incomplete number
	return tuple(round - 1, combatants.alive.map!(c => c.health).sum);
}

void main(string[] args)
{
	char[][] grid = readText(args[1]).splitLines.map!(s => s.dup).array;
	auto result = playGame(grid);
	writeln("Final score after ", result[0], " full turns: ", result[0] * result[1], " (remaining health ", result[1], "hp)");
}

// A* test
unittest
{
	char[][] grid = ["####",
	                 "#..#",
	                 "#.##",
	                 "#..#",
	                 "##.#",
	                 "#..#",
	                 "####"].map!(s => s.dup).array;

	auto expected = [Coord(2, 1), Coord(1, 1), Coord(1, 2), Coord(1, 3), Coord(2, 3), Coord(2, 4), Coord(2, 5), Coord(1, 5)];
	auto got = AStar(grid, Coord(2, 1), Coord(1, 5));
	assert(got == expected, "\nExpected: %s\nGot:      %s".format(expected, got));
}

// A* unreachable
unittest
{
	char[][] grid = ["####",
	                 "#..#",
	                 "####",
	                 "#..#",
	                 "####"].map!(s => s.dup).array;

	auto expected = [];
	auto got = AStar(grid, Coord(1, 1), Coord(1, 3));
	assert(got == expected, "\nExpected: %s\nGot:      %s".format(expected, got));
}

// Test cases
unittest
{
	auto grid = [
		"#######",
		"#.G...#",
		"#...EG#",
		"#.#.#G#",
		"#..G#E#",
		"#.....#",
		"#######",
	].map!(s => s.dup).array;
	auto expected = tuple(47, 590);
	auto got = playGame(grid);
	assert(got == expected, "Expected: %s, Got: %s".format(expected, got));
}

unittest
{
	auto grid = [
		"#######",
		"#G..#E#",
		"#E#E.E#",
		"#G.##.#",
		"#...#E#",
		"#...E.#",
		"#######",
	].map!(s => s.dup).array;
	auto expected = tuple(37, 982);
	auto got = playGame(grid);
	assert(got == expected, "Expected: %s, Got: %s".format(expected, got));
}

unittest
{
	auto grid = [
		"#######",
		"#E..EG#",
		"#.#G.E#",
		"#E.##E#",
		"#G..#.#",
		"#..E#.#",
		"#######",
	].map!(s => s.dup).array;
	auto expected = tuple(46, 859);
	auto got = playGame(grid);
	assert(got == expected, "Expected: %s, Got: %s".format(expected, got));
}

unittest
{
	auto grid = [
		"#######",
		"#E.G#.#",
		"#.#G..#",
		"#G.#.G#",
		"#G..#.#",
		"#...E.#",
		"#######",
	].map!(s => s.dup).array;
	auto expected = tuple(35, 793);
	auto got = playGame(grid);
	assert(got == expected, "Expected: %s, Got: %s".format(expected, got));
}

unittest
{
	auto grid = [
		"#######",
		"#.E...#",
		"#.#..G#",
		"#.###.#",
		"#E#G#G#",
		"#...#G#",
		"#######",
	].map!(s => s.dup).array;
	auto expected = tuple(54, 536);
	auto got = playGame(grid);
	assert(got == expected, "Expected: %s, Got: %s".format(expected, got));
}

unittest
{
	auto grid = [
		"#########",
		"#G......#",
		"#.E.#...#",
		"#..##..G#",
		"#...##..#",
		"#...#...#",
		"#.G...G.#",
		"#.....G.#",
		"#########",
	].map!(s => s.dup).array;
	auto expected = tuple(20, 937);
	auto got = playGame(grid);
	assert(got == expected, "Expected: %s, Got: %s".format(expected, got));
}


unittest
{
	auto grid = [
		"####",
		"##E#",
		"#GG#",
		"####",
	].map!(s => s.dup).array;
	auto expected = tuple(67, 200); // TODO: Verify
	auto got = playGame(grid);
	assert(got == expected, "Expected: %s, Got: %s".format(expected, got));
}

unittest
{
	auto grid = [
		"#####",
		"#GG##",
		"#.###",
		"#..E#",
		"#.#G#",
		"#.E##",
		"#####",
	].map!(s => s.dup).array;
	auto expected = tuple(71, 197); // TODO: Verify
	auto got = playGame(grid);
	assert(got == expected, "Expected: %s, Got: %s".format(expected, got));
}

unittest
{
	auto grid = [
		"#######",
		"#.E..G#",
		"#.#####",
		"#G#####",
		"#######",
	].map!(s => s.dup).array;
	auto expected = tuple(34, 301); // TODO: Verify
	auto got = playGame(grid);
	assert(got == expected, "Expected: %s, Got: %s".format(expected, got));
}
