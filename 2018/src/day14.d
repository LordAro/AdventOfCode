import std.algorithm : maxElement;
import std.file : readText;
import std.stdio : writeln;
import std.range;
import std.string;
import std.algorithm;
import std.conv;

void main(string[] args)
{
	auto input = readText(args[1]).strip;
	auto input_limit = input.to!ulong;

	ubyte[] input_array = input.map!(c => cast(ubyte)(c - '0')).array;

	ubyte[] recipes = [3, 7];
	recipes.reserve(input_limit + 10);

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
		if (recipes.length == input_limit + 10) {
			writeln("Recipes: ", cast(string)recipes[$-10 .. $].map!(c => c + '0').array);
			reached_limit = true;
		}
		if (!found_subset && recipes.length > input_array.length) {
			if (recipes[$-input_array.length - 1 .. $ - 1] == input_array) {
				writeln("Found pattern at position ", recipes.length - input_array.length - 1);
				found_subset = true;
			} else if (recipes[$-input_array.length - 1 .. $ - 1] == input_array) {
				writeln("Found pattern at position ", recipes.length - input_array.length - 1);
				found_subset = true;
			}
		}
	}
}
