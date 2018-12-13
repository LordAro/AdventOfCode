import std.algorithm : map, filter, each, count, sort, find;
import std.range : array, front;
import std.stdio : writeln;
import std.file : readText;
import std.string : splitLines;
import std.typecons : Tuple, tuple;

enum Direction {
	North,
	East,
	South,
	West,
}

enum Turn {
	Left,
	Straight,
	Right,
}

struct Cart {
	Tuple!(ulong, "x", ulong, "y") pos;
	Direction dir;
	Turn nextturn = Turn.Left;
	bool crashed = false;
}

Direction make_turn(Direction d, Turn t)
{
	final switch (t) {
		case Turn.Left:
			final switch (d) {
				case Direction.North: return Direction.West;
				case Direction.East:  return Direction.North;
				case Direction.South: return Direction.East;
				case Direction.West:  return Direction.South;
			}
		case Turn.Straight:
			return d;
		case Turn.Right:
			final switch (d) {
				case Direction.North: return Direction.East;
				case Direction.East:  return Direction.South;
				case Direction.South: return Direction.West;
				case Direction.West:  return Direction.North;
			}
	}
}

// Debugging
void print_grid(char[][] grid, Cart[] carts)
{
	char dir_to_sym(Direction d)
	{
		final switch (d) {
			case Direction.North: return '^';
			case Direction.East:  return '>';
			case Direction.South: return 'v';
			case Direction.West:  return '<';
		}
	}
	auto copy = grid.map!(r => r.dup).array;
	carts.filter!(c => c.crashed).each!(c => copy[c.pos.y][c.pos.x] = 'X');
	carts.filter!(c => !c.crashed).each!(c => copy[c.pos.y][c.pos.x] = dir_to_sym(c.dir));
	copy.each!writeln;
}

void main(string[] args)
{
	char[][] grid = readText(args[1]).splitLines.map!(s => s.dup).array;

	Cart[] carts;
	foreach (y, row; grid) {
		foreach (x, c; row) {
			if (c == '^' || c == '>' || c == 'v' || c == '<') {
				Direction d;
				final switch (c) {
					case '^': d = Direction.North; break;
					case '>': d = Direction.East;  break;
					case 'v': d = Direction.South; break;
					case '<': d = Direction.West;  break;
				}
				Cart cart = {tuple(x, y), d};
				carts ~= cart;
				// Put the actual grid piece in
				final switch (c) {
					case '^', 'v':
						grid[y][x] = '|';
						break;
					case '<', '>':
						grid[y][x] = '-';
						break;
				}
			}
		}
	}

	while (carts.count!(c => !c.crashed) > 1) {
		carts.sort!((c, d) => c.pos < d.pos); // Make sure movements happen top to bottom, left to right
		foreach (ref c; carts) {
			if (c.crashed) continue; // Can't use filter, as crashes also update the other cart
			// Move cart
			final switch (c.dir) {
				case Direction.North: c.pos = tuple(c.pos.x, c.pos.y - 1); break;
				case Direction.East:  c.pos = tuple(c.pos.x + 1, c.pos.y); break;
				case Direction.South: c.pos = tuple(c.pos.x, c.pos.y + 1); break;
				case Direction.West:  c.pos = tuple(c.pos.x - 1, c.pos.y); break;
			}

			// Crash?
			if (carts.filter!(d => !d.crashed).count!(d => d.pos == c.pos) > 1) {
				if (carts.filter!(d => d.crashed).count == 0) { // First crash
					writeln("First crash at ", c.pos.x, ",", c.pos.y);
				}
				carts.filter!(d => d.pos == c.pos).each!((ref c) => c.crashed = true);
			}

			// Change direction if moved onto a turn/junction
			switch (grid[c.pos.y][c.pos.x]) {
				case '/':
					final switch (c.dir) {
						case Direction.North: c.dir = Direction.East;  break;
						case Direction.East:  c.dir = Direction.North; break;
						case Direction.South: c.dir = Direction.West;  break;
						case Direction.West:  c.dir = Direction.South; break;
					}
					break;
				case '\\':
					final switch (c.dir) {
						case Direction.North: c.dir = Direction.West;  break;
						case Direction.East:  c.dir = Direction.South; break;
						case Direction.South: c.dir = Direction.East;  break;
						case Direction.West:  c.dir = Direction.North; break;
					}
					break;
				case '+':
					c.dir = make_turn(c.dir, c.nextturn);
					final switch (c.nextturn) {
						case Turn.Left:     c.nextturn = Turn.Straight; break;
						case Turn.Straight: c.nextturn = Turn.Right;    break;
						case Turn.Right:    c.nextturn = Turn.Left;     break;
					}
					break;
				case '|', '-':
					break;
				default:
					writeln("'", grid[c.pos.y][c.pos.x], "'");
					writeln(c);
					assert(0, "Cart off the rails");
			}
		}
	}
	auto coord = carts.find!(c => !c.crashed).front.pos;
	writeln("Coord of last remaining cart: ", coord.x, ",", coord.y);
}
