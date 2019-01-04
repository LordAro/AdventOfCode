import std.algorithm : count, maxElement, min, max, each, filter, map;
import std.container : RedBlackTree;
import std.math : abs;
import std.stdio : writeln, readln;
import std.file : readText;
import std.string : splitLines;
import std.typecons : Tuple, tuple;
import std.format : formattedRead;
import std.range : front, empty, dropOne;

alias Coord = Tuple!(long, "x", long, "y", long, "z");
alias Cube = Tuple!(Coord, Coord);
alias Nanobot = Tuple!(long, "x", long, "y", long, "z", long, "r");

long manhattan(T, S)(T a, S b)
{
	return abs(a.x - b.x) + abs(a.y - b.y) + abs(a.z - b.z);
}

bool cube_intersects_sphere(Coord c1, Coord c2, Nanobot b)
{
	if (c1.x <= b.x && b.x <= c2.x && c1.y <= b.y && b.y <= c2.y && c1.z <= b.z && b.z <= c2.z) {
		return true;
	} else if (c1.x <= b.x && b.x <= c2.x && c1.y <= b.y && b.y <= c2.y) {
		return (c1.z - b.r) <= b.z && b.z <= (c2.z + b.r);
	} else if (c1.x <= b.x && b.x <= c2.x && c1.z <= b.z && b.z <= c2.z) {
		return (c1.y - b.r) <= b.y && b.y <= (c2.y + b.r);
	} else if (c1.y <= b.y && b.y <= c2.y && c1.z <= b.z && b.z <= c2.z) {
		return (c1.x - b.r) <= b.x && b.x <= (c2.x + b.r);
	} else {
		foreach (xp; [c1.x, c2.x-1]) {
			foreach (yp; [c1.y, c2.y-1]) {
				foreach (zp; [c1.z, c2.z-1]) {
					if (manhattan(Coord(xp, yp, zp), b) <= b.r) {
						return true;
					}
				}
			}
		}
	}
	return false;
}

void main(string[] args)
{
	auto input = readText(args[1]).splitLines;
	Nanobot[] bots;
	foreach (line; input) {
		Nanobot bot;
		line.formattedRead("pos=<%d,%d,%d>, r=%d", &bot.x, &bot.y, &bot.z, &bot.r);
		bots ~= bot;
	}

	auto most_powerful = bots.maxElement!(a => a.r);
	auto in_range = bots.count!(a => manhattan(a, most_powerful) <= most_powerful.r);
	writeln("Number of bots in range: ", in_range);

	auto cmin = Coord(long.max, long.max, long.max);
	auto cmax = Coord(long.min, long.min, long.min);
	foreach (b; bots) {
		cmin.x = min(b.x, cmin.x);
		cmin.y = min(b.y, cmin.y);
		cmin.z = min(b.z, cmin.z);
		cmax.x = max(b.x, cmax.x);
		cmax.y = max(b.y, cmax.y);
		cmax.z = max(b.z, cmax.z);
	}

	long max_bots;
	Coord max_position;
	auto search_cubes = new RedBlackTree!(Tuple!(Cube, ulong), (a, b) => a[1] > b[1])(tuple(Cube(cmin, cmax), bots.length));
	while (!search_cubes.empty) {
		//foreach (s; search_cubes) {
		//	writeln(s[0][0].x, ',', s[0][0].y, ',', s[0][0].z, " -> ", s[0][1].x, ',', s[0][1].y, ',', s[0][1].z, ": ", s[1]);
		//}
		auto cube = search_cubes.front;
		search_cubes.removeFront();
		cmin = cube[0][0];
		cmax = cube[0][1];
		ulong num_bots;

		while (cmin != cmax) {
			auto cpar = Coord(cmin.x + (cmax.x - cmin.x) / 2,
							  cmin.y + (cmax.y - cmin.y) / 2,
							  cmin.z + (cmax.z - cmin.z) / 2);
			ulong[Cube] buckets;
			if (cmin.x != cmax.x && cmin.y != cmax.y && cmin.z != cmax.z) {
				buckets[Cube(Coord(cmin.x,     cmin.y,     cmin.z), Coord(cpar.x, cpar.y, cpar.z))] = 0;
				buckets[Cube(Coord(cpar.x + 1, cmin.y,     cmin.z), Coord(cmax.x, cpar.y, cpar.z))] = 0;
				buckets[Cube(Coord(cmin.x,     cpar.y + 1, cmin.z), Coord(cpar.x, cmax.y, cpar.z))] = 0;
				buckets[Cube(Coord(cpar.x + 1, cpar.y + 1, cmin.z), Coord(cmax.x, cmax.y, cpar.z))] = 0;

				buckets[Cube(Coord(cmin.x,     cmin.y,     cpar.z + 1), Coord(cpar.x, cpar.y, cmax.z))] = 0;
				buckets[Cube(Coord(cpar.x + 1, cmin.y,     cpar.z + 1), Coord(cmax.x, cpar.y, cmax.z))] = 0;
				buckets[Cube(Coord(cmin.x,     cpar.y + 1, cpar.z + 1), Coord(cpar.x, cmax.y, cmax.z))] = 0;
				buckets[Cube(Coord(cpar.x + 1, cpar.y + 1, cpar.z + 1), Coord(cmax.x, cmax.y, cmax.z))] = 0;
			} else if (cmin.x != cmax.x && cmin.y != cmax.y) {
				buckets[Cube(Coord(cmin.x,     cmin.y,     cmin.z), Coord(cpar.x, cpar.y, cmax.z))] = 0;
				buckets[Cube(Coord(cpar.x + 1, cmin.y,     cmin.z), Coord(cmax.x, cpar.y, cmax.z))] = 0;
				buckets[Cube(Coord(cmin.x,     cpar.y + 1, cmin.z), Coord(cpar.x, cmax.y, cmax.z))] = 0;
				buckets[Cube(Coord(cpar.x + 1, cpar.y + 1, cmin.z), Coord(cmax.x, cmax.y, cmax.z))] = 0;
			} else if (cmin.x != cmax.x && cmin.z != cmax.z) {
				buckets[Cube(Coord(cmin.x,     cmin.y, cmin.z    ), Coord(cpar.x, cmax.y, cpar.z))] = 0;
				buckets[Cube(Coord(cpar.x + 1, cmin.y, cmin.z    ), Coord(cmax.x, cmax.y, cpar.z))] = 0;
				buckets[Cube(Coord(cmin.x,     cmin.y, cpar.z + 1), Coord(cpar.x, cmax.y, cmax.z))] = 0;
				buckets[Cube(Coord(cpar.x + 1, cmin.y, cpar.z + 1), Coord(cmax.x, cmax.y, cmax.z))] = 0;
			} else if (cmin.y != cmax.y && cmin.z != cmax.z) {
				buckets[Cube(Coord(cmin.x, cmin.y,     cmin.z    ), Coord(cmax.x, cpar.y, cpar.z))] = 0;
				buckets[Cube(Coord(cmin.x, cpar.y + 1, cmin.z    ), Coord(cmax.x, cmax.y, cpar.z))] = 0;
				buckets[Cube(Coord(cmin.x, cmin.y,     cpar.z + 1), Coord(cmax.x, cpar.y, cmax.z))] = 0;
				buckets[Cube(Coord(cmin.x, cpar.y + 1, cpar.z + 1), Coord(cmax.x, cmax.y, cmax.z))] = 0;
			} else if (cmin.x != cmax.x) {
				buckets[Cube(Coord(cmin.x,     cmin.y, cmin.z), Coord(cpar.x, cmax.y, cmax.z))] = 0;
				buckets[Cube(Coord(cpar.x + 1, cmin.y, cmin.z), Coord(cmax.x, cmax.y, cmax.z))] = 0;
			} else if (cmin.y != cmax.y) {
				buckets[Cube(Coord(cmin.x, cmin.y,     cmin.z), Coord(cmax.x, cpar.y, cmax.z))] = 0;
				buckets[Cube(Coord(cmin.x, cpar.y + 1, cmin.z), Coord(cmax.x, cmax.y, cmax.z))] = 0;
			} else if (cmin.z != cmax.z) {
				buckets[Cube(Coord(cmin.x, cmin.y, cmin.z    ), Coord(cmax.x, cmax.y, cpar.z))] = 0;
				buckets[Cube(Coord(cmin.x, cmin.y, cpar.z + 1), Coord(cmax.x, cmax.y, cmax.z))] = 0;
			}

			//int[Cube] buckets = [
			//	Cube(Coord(cmin.x, cmin.y, cmin.z), Coord(cpar.x, cpar.y, cpar.z)): 0,
			//	Cube(Coord(cpar.x, cmin.y, cmin.z), Coord(cmax.x, cpar.y, cpar.z)): 0,
			//	Cube(Coord(cmin.x, cpar.y, cmin.z), Coord(cpar.x, cmax.y, cpar.z)): 0,
			//	Cube(Coord(cpar.x, cpar.y, cmin.z), Coord(cmax.x, cmax.y, cpar.z)): 0,

			//	Cube(Coord(cmin.x, cmin.y, cpar.z), Coord(cpar.x, cpar.y, cmax.z)): 0,
			//	Cube(Coord(cpar.x, cmin.y, cpar.z), Coord(cmax.x, cpar.y, cmax.z)): 0,
			//	Cube(Coord(cmin.x, cpar.y, cpar.z), Coord(cpar.x, cmax.y, cmax.z)): 0,
			//	Cube(Coord(cpar.x, cpar.y, cpar.z), Coord(cmax.x, cmax.y, cmax.z)): 0,
			//];

			foreach (b; bots) {
				foreach (s; buckets.byKey) {
					if (cube_intersects_sphere(s[0], s[1], b)) buckets[s]++;
				}
			}
			//foreach (s; buckets.byKey) {
			//	writeln(s[0].x, ',', s[0].y, ',', s[0].z, " -> ", s[1].x, ',', s[1].y, ',', s[1].z, ": ", buckets[s]);
			//}
			auto max_val = buckets.byKeyValue.maxElement!(b => b.value).value;
			if (max_val < max_bots) break; // Fewer bots in this region than our current best point, abort
			auto next_key = buckets.byKeyValue
				.filter!(b => b.value == max_val)
				.map!(b => b.key).front;
			buckets.byKeyValue.filter!(b => b.key != next_key).each!(b => search_cubes.insert(tuple(b.key, max_val)));
			cmin = next_key[0];
			cmax = next_key[1];
			num_bots = max_val;
		}
	//	writeln(cmin.x, ',', cmin.y, ',', cmin.z, ": ", num_bots);
		if (num_bots > max_bots) {
			max_bots = num_bots;
			max_position = cmin;
			auto manhattan_dist = manhattan(Coord(0, 0, 0), max_position);
			writeln(cmin.x, ',', cmin.y, ',', cmin.z, ": ", max_bots, ',', manhattan_dist);
		}
	}
}
