import std.algorithm;
import std.conv;
import std.container;
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

ulong[Coord] erosion_cache; // Global cache to save recomputing
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

ulong find_route_time(CoordItem start, CoordItem end, ulong depth)
{
	ulong[CoordItem] searched;

	// Order by distance from source
	bool search_ordering(Tuple!(CoordItem, ulong) a, Tuple!(CoordItem, ulong) b)
	{
		return a[1] == b[1] ? a[0] < b[0] : a[1] < b[1];
	}
	auto toSearch = new RedBlackTree!(Tuple!(CoordItem, ulong), search_ordering)(tuple(start, 0uL));

	while (end !in searched) {
		Tuple!(CoordItem, ulong) current_t = toSearch.front;
		CoordItem current = current_t[0];
		ulong cur_score = current_t[1];
		toSearch.removeFront();

		if (current in searched) continue; // Duplicate, but we've already processed a shorter route
		searched[current] = cur_score;

		auto neighbours = chain(
			adjacent_coords(current.c).map!(c => CoordItem(c, current.item)),
			[CoordItem(current.c, compatible_items(current.c, end.c, depth)
				.filter!(i => i != current.item).front)]
		);
		foreach (n; neighbours) {
			if (!compatible_items(n.c, end.c, depth).canFind(n.item)) continue;

			if (n !in searched) {
				toSearch.insert(tuple(n, cur_score + (n.item != current.item ? 7 : 1)));
			}
		}
	}
	return searched[end];
}

void main(string[] args)
{
	auto input = File(args[1]).byLine;
	ulong depth = input.front.split.back.to!ulong;
	auto target_r = input.dropOne.front.split.back.split(",").map!(n => n.to!ulong);
	Coord target = Coord(target_r[0], target_r[1]);

	erosion_cache.clear(); // Unit tests retain state...
	writeln("Risk level: ", risk_level(depth, Coord(0, 0), target));

	auto route_time = find_route_time(CoordItem(Coord(0, 0), Item.Torch), CoordItem(target, Item.Torch), depth);
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

unittest
{
	assert(find_route_time(CoordItem(Coord(0, 0), Item.Torch, CoordItem(Coord(10, 10), Item.Torch), 510)) == 45);
}
