import std.algorithm;
import std.conv;
import std.file;
import std.stdio;
import std.typecons;
import std.string;
import std.range;
import std.math : abs;

alias Coord = Tuple!(ulong, "x", ulong, "y");

enum Item {
	None,
	Torch,
	Climbing,
}

enum RockType {
	Rocky,
	Wet,
	Narrow,
}

RockType get_type(ulong c)
{
	return cast(RockType)(c % 3);
}

ulong[Coord] erosion_cache;
ulong erosion_level(ulong depth, Coord c, Coord target)
{
	if (c in erosion_cache) return erosion_cache[c];
	ulong new_val = (depth + geologic_index(depth, c, target)) % 20183;
	erosion_cache[c] = new_val;
	return new_val;
}

ulong geologic_index(ulong depth, Coord c, Coord target)
{
	if (c == Coord(0, 0) || c == target) return 0;
	if (c.y == 0) return c.x * 16807;
	if (c.x == 0) return c.y * 48271;

	return erosion_level(depth, Coord(c.x - 1, c.y), target) * erosion_level(depth, Coord(c.x, c.y - 1), target);
}

ulong risk_level(ulong depth, Coord start, Coord end)
{
	ulong risk_level;
	for (ulong j = start.y; j <= end.y; j++) {
		for (ulong i = start.x; i <= end.x; i++) {
			risk_level += erosion_level(depth, Coord(i, j), end) % 3;
		}
	}
	return risk_level;
}

auto adjacent_coords(Coord a)
{
	return [Coord(a.x, a.y - 1), Coord(a.x - 1, a.y), Coord(a.x + 1, a.y), Coord(a.x, a.y + 1)]
		.filter!(c => c.x != ulong.max && c.y != ulong.max);
}

auto compatible_items(Coord a, Coord target, ulong depth)
{
	final switch (get_type(erosion_level(depth, a, target))) {
		case RockType.Rocky:
			return [Item.Climbing, Item.Torch];
		case RockType.Wet:
			return [Item.Climbing, Item.None];
		case RockType.Narrow:
			return [Item.Torch, Item.None];
	}
}

ulong manhattan(Coord a, Coord b)
{
	return abs(cast(int)a.x - cast(int)b.x) + abs(cast(int)a.y - cast(int)b.y);
}

alias CoordItem = Tuple!(Coord, "c", Item, "item");

string toString(CoordItem ci)
{
	return "%d,%d %s".format(ci.c.x, ci.c.y, ci.item);
}

CoordItem[] AStar(CoordItem start, CoordItem end, ulong depth)
{
	CoordItem[] searched;
	CoordItem[] toSearch = [start];

	CoordItem[CoordItem] cameFrom;

	ulong[CoordItem] gScore;
	gScore[start] = 0;

	ulong[CoordItem] fScore;
	fScore[start] = manhattan(start.c, end.c);

	while(!toSearch.empty) {
		CoordItem current = toSearch.minElement!(n => n in fScore ? fScore[n] : ulong.max);
		if (current == end) {
			//gScore.byKeyValue.each!(t => writeln("%s: %d".format(toString(t.key), t.value)));
			// Reconstruct path
			CoordItem[] total_path = [current];
			while (current in cameFrom) {
				current = cameFrom[current];
				total_path ~= current;
			}
			return total_path.reverse;
		}

		toSearch = toSearch.remove!(a => a == current);
		searched ~= current;

		auto neighbours = chain(
			adjacent_coords(current.c).map!(c => CoordItem(c, current.item)),
			[CoordItem(current.c, compatible_items(current.c, end.c, depth)
				.filter!(i => i != current.item).front)]
		);
		//writeln(current.c.x, ",", current.c.y, " ", current.item);
		//neighbours.each!(a => writeln(a.c.x, ",", a.c.y, " ", a.item));
		//writeln();
		foreach (n; neighbours) {
			if (searched.canFind(n)) continue;
			if (!compatible_items(n.c, end.c, depth).canFind(n.item)) continue;

			ulong tentative = gScore[current] + (n.item != current.item ? 7 : 1);
			if (!toSearch.canFind(n)) {
				toSearch ~= n;
			} else if (tentative >= (n in gScore ? gScore[n] : ulong.max)) {
				continue;
			}

			cameFrom[n] = current;
			gScore[n] = tentative;
			fScore[n] = gScore[n] + manhattan(n.c, end.c) + 7;
		}
	}
	assert(false); // Should always be a path
}

void main(string[] args)
{
	auto input = File(args[1]).byLine;
	ulong depth = input.front.split.back.to!ulong;
	auto target_r = input.dropOne.front.split.back.split(",").map!(n => n.to!ulong);
	Coord target = Coord(target_r[0], target_r[1]);
	//ulong depth = 510;
	//Coord target = Coord(10, 10);

	erosion_cache.clear();
	writeln("Risk level: ", risk_level(depth, Coord(0, 0), target));

	auto route = AStar(CoordItem(Coord(0, 0), Item.Torch), CoordItem(target, Item.Torch), depth);
	route.each!(a => writeln(a.c.x, ",", a.c.y, " ", a.item));
	ulong route_time = 0;
	Item cur_item = route.front.item;
	foreach (c; route.dropOne) {
		if (c.item != cur_item) {
			route_time += 7;
			cur_item = c.item;
		} else {
			route_time += 1;
		}
	}
	writeln("Route time: ", route_time);
}

// Base cases
unittest
{
	assert(geologic_index(510, Coord(0, 0), Coord(10, 10)) == 0);
	assert(erosion_level(510, Coord(0, 0), Coord(10, 10)) == 510);
	assert(geologic_index(510, Coord(1, 0), Coord(10, 10)) == 16807);
	assert(erosion_level(510, Coord(1, 0), Coord(10, 10)) == 17317);
	assert(geologic_index(510, Coord(0, 1), Coord(10, 10)) == 48271);
	assert(erosion_level(510, Coord(0, 1), Coord(10, 10)) == 8415);
	assert(geologic_index(510, Coord(10, 10), Coord(10, 10)) == 0);
	assert(erosion_level(510, Coord(10, 10), Coord(10, 10)) == 510);
}

unittest
{
	assert(geologic_index(510, Coord(1, 1), Coord(10, 10)) == 145722555);
	assert(erosion_level(510, Coord(1, 1), Coord(10, 10)) == 1805);
}

unittest
{
	assert(risk_level(510, Coord(0, 0), Coord(10, 10)) == 114);
}
