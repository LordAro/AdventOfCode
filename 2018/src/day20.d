import std.algorithm : count, filter, min, maxElement;
import std.stdio : writeln;
import std.container : redBlackTree, RedBlackTree;
import std.range : back, dropBackOne, dropOne, popBack;
import std.file : readText;
import std.typecons : Tuple;

alias Coord = Tuple!(int, "x", int, "y");

void main(string[] args)
{
	string input = readText(args[1]);

	Coord[] positions;
	RedBlackTree!(Coord)[Coord] m;
	Coord pos, prev_pos;
	int[Coord] distances = [pos: 0];

	// drop ^ and $ (and trailing newline)
	foreach (c; input.dropOne.dropBackOne.dropBackOne) {
		switch (c) {
			case '(':
				positions ~= pos;
				break;
			case ')':
				pos = positions.back;
				positions.popBack();
				break;
			case '|':
				pos = positions.back;
				break;
			default:
				final switch (c) {
					case 'N': pos = Coord(pos.x, pos.y - 1); break;
					case 'E': pos = Coord(pos.x + 1, pos.y); break;
					case 'S': pos = Coord(pos.x, pos.y + 1); break;
					case 'W': pos = Coord(pos.x - 1, pos.y); break;
				}
				if (pos !in m) {
					m[pos] = redBlackTree!Coord;
				}
				m[pos].insert(prev_pos);
				if (pos in distances && distances[pos] != 0) {
					distances[pos] = min(distances[pos], distances[prev_pos] + 1);
				} else {
					distances[pos] = distances[prev_pos] + 1;
				}
				break;
		}
		prev_pos = pos;
	}

	writeln("Max length: ", distances.values.maxElement);
	writeln("Number of distant rooms: ", distances.values.filter!(a => a >= 1000).count);
}
