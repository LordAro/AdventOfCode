import std.file : readText;
import std.stdio : writeln;
import std.range : array;
import std.string : strip;
import std.algorithm : map;
import std.conv : to;

void main(string[] args)
{
	auto input = readText(args[1]).strip;
	auto input_limit = input.to!ulong;

	ubyte[] input_array = input.map!(c => cast(ubyte)(c - '0')).array;

	ubyte[] recipes = [3, 7];

	ulong elf1 = 0;
	ulong elf2 = 1;
	bool found_subset = false;
	bool reached_limit = false;
	while (!found_subset || !reached_limit) {
		ubyte r_sum = cast(ubyte)(recipes[elf1] + recipes[elf2]);
		if (r_sum >= 10) {
			recipes ~= r_sum / 10;
			recipes ~= r_sum % 10;
		} else {
			recipes ~= r_sum;
		}
		elf1 = (elf1 + 1 + recipes[elf1]) % recipes.length;
		elf2 = (elf2 + 1 + recipes[elf2]) % recipes.length;

		// Avoid possibility of off by one error by checking both conditions
		if (!reached_limit && recipes.length == input_limit + 10) {
			writeln("Recipes: ", cast(string)recipes[$ - 10 .. $].map!(c => c + '0').array);
			reached_limit = true;
		} else if (!reached_limit && recipes.length == input_limit + 10 + 1) {
			writeln("Recipes: ", cast(string)recipes[$ - 10 - 1 .. $ - 1].map!(c => c + '0').array);
			reached_limit = true;
		}

		// Part2
		if (!found_subset && recipes.length > input_array.length) {
			if (recipes[$ - input_array.length .. $] == input_array) {
				writeln("Found pattern at position ", recipes.length - input_array.length);
				found_subset = true;
			} else if (recipes[$ - input_array.length - 1 .. $ - 1] == input_array) {
				writeln("Found pattern at position ", recipes.length - input_array.length - 1);
				found_subset = true;
			}
		}
	}
}
