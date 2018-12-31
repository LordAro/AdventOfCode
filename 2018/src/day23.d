import std.algorithm : count, maxElement, min, max;
import std.math : abs;
import std.stdio : writeln;
import std.file : readText;
import std.string : splitLines;
import std.typecons : Tuple;
import std.format : formattedRead;

alias Nanobot = Tuple!(int, "x", int, "y", int, "z", int, "r");

int manhattan(Nanobot a, Nanobot b)
{
	return abs(a.x - b.x) + abs(a.y - b.y) + abs(a.z - b.z);
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
}
