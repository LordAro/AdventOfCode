import std.conv;
import std.file;
import std.regex;
import std.stdio;

struct claim {
	size_t id;
	size_t x;
	size_t y;
	size_t w;
	size_t h;
}
void main(string[] args)
{
	auto lines = slurp!(string)(args[1], "%s");
	int[1000][1000] cloth;
	auto reg = regex(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)");
	claim[] claims;
	foreach (claim_str; lines) {
		auto c = matchFirst(claim_str, reg);
		size_t id = to!size_t(c[1]);
		size_t x = to!size_t(c[2]);
		size_t y = to!size_t(c[3]);
		size_t w = to!size_t(c[4]);
		size_t h = to!size_t(c[5]);
		claims ~= claim(id, x, y, w, h);
		for (size_t i = x; i < x+w; i++) {
			for (size_t j = y; j < y+h; j++) {
				cloth[j][i]++;
			}
		}
	}

	int count = 0;
	foreach (row; cloth) {
		foreach (cell; row) {
			if (cell >= 2) {
				count++;
			}
		}
	}
	writeln("Duplicate claim space: ", count);

	foreach (c; claims) {
		size_t nonoverlapped_id = c.id;
		outer: for (size_t i = c.x; i < c.x+c.w; i++) {
			for (size_t j = c.y; j < c.y+c.h; j++) {
				if (cloth[j][i] > 1) {
					nonoverlapped_id = 0;
					break outer;
				}
			}
		}
		if (nonoverlapped_id != 0) {
			writeln("Nonoverlapping id: ", nonoverlapped_id);
			break; // Just the one
		}
	}
}
