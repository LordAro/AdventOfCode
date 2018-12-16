import std.array : popFront;
import std.algorithm : canFind, count, filter, joiner, map, minElement, minPos, sort, sum;
import std.container: DList;
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

// Modified Dijkstra/BFS. returns all targets with equal distance
Tuple!(Coord[], ulong) find_closest(char[][] grid, Coord start, Coord[] targets)
{
	Coord[] searched;

	auto toSearch = DList!(Tuple!(Coord, ulong))(tuple(start, 0uL));
	ulong found_dist = grid.length * grid.length;
	Coord[] closest;

	while (!toSearch.empty) {
		Tuple!(Coord, ulong) current = toSearch.front;
		toSearch.removeFront();

		if (current[1] > found_dist) {
			break;
		}
		if (searched.canFind(current[0]) || (current[0] != start && grid[current[0].y][current[0].x] != '.')) continue;

		searched ~= current[0];

		if (targets.canFind(current[0])) {
			found_dist = current[1];
			closest ~= current[0];
		}

		foreach (n; grid.blank_adjacents(current[0])) {
			if (searched.canFind(n)) continue;
			toSearch ~= tuple(n, current[1] + 1);
		}
	}

	return tuple(closest, found_dist);
}

bool reading_order(Coord a, Coord b)
{
	return a.y < b.y || (a.y == b.y && a.x < b.x);
}

Tuple!(int, int, ulong) playGame(char[][] grid, int elfAttackDamage = 3, bool allowElfDeath = true)
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
			if (p.get_adjacent(combatants).empty) {
				// Choose shortest distance, or reading order, or reading order of first move
				Coord[] targets = combatants.alive.filter!(c => c.isElf != p.isElf).map!(c => grid.blank_adjacents(c.pos)).joiner.array;

				auto closest_targets = find_closest(grid, p.pos, targets);

				if (!closest_targets[0].empty) {
					auto closest_target = closest_targets[0].minElement!(reading_order);
					foreach (n; grid.blank_adjacents(p.pos)) { // adjacents already sorted in reading order
						auto route = find_closest(grid, n, [closest_target]);
						if (route[1] == closest_targets[1] - 1) {
							move_combatant(grid, p, n);
							break;
						}
					}
				}
			}

			// Attack
			auto adjacents = p.get_adjacent(combatants);
			if (!adjacents.empty) {
				// Attack adjacent with lowest health, or reading order if equal
				// Must use minPos to get reference
				auto target = adjacents.minPos!((p, q) => p.health < q.health || (p.health == q.health && reading_order(p.pos, q.pos)));
				target.front.health -= p.isElf ? elfAttackDamage : 3;
				if (target.front.health <= 0) {
					grid[target.front.pos.y][target.front.pos.x] = '.';
					if (target.front.isElf && !allowElfDeath) {
						break outer;
					}
				}
			}
		}

		//writeln("After round ", round);
		//print_grid(grid, combatants);
		//writeln();
	}

	// tuple(round, remaining health, number of elf deaths) - round always ends on the incomplete number
	return tuple(round - 1, combatants.alive.map!(c => c.health).sum, combatants.elves.count!(c => c.health <= 0));
}

void main(string[] args)
{
	immutable char[][] grid = readText(args[1]).splitLines;
	auto result = playGame(grid.map!(s => s.dup).array);
	writeln("Final score after ", result[0], " full turns: ", result[0] * result[1], " (remaining health ", result[1], "hp)");

	for (int i = 4; ; i++) {
		result = playGame(grid.map!(s => s.dup).array, i, false);
		if (result[2] == 0) {
			writeln("Final score with no elf deaths: ", result[0] * result[1], " (with ", i, " attack)");
			break;
		}
	}
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
	auto expected = tuple(47, 590, 2);
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
	auto expected = tuple(37, 982, 1);
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
	auto expected = tuple(46, 859, 1);
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
	auto expected = tuple(35, 793, 2);
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
	auto expected = tuple(54, 536, 2);
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
	auto expected = tuple(20, 937, 1);
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
	auto expected = tuple(67, 200, 1);
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
	auto expected = tuple(71, 197, 2);
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
	auto expected = tuple(34, 301, 1);
	auto got = playGame(grid);
	assert(got == expected, "Expected: %s, Got: %s".format(expected, got));
}
