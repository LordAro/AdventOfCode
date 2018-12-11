import std.conv : to;
import std.algorithm : max;
import std.file : readText;
import std.stdio : writeln;
import std.string : strip;

pure int power_level(int x, int y, int serial)
{
	int rackid = x + 10;
	return (((rackid * y + serial) * rackid / 100) % 10) - 5;
}

void main(string[] args)
{
	auto serial = readText(args[1]).strip.to!int;

	int [300][300] sum_grid;
	for (int x = 0; x < 300; x++) {
		for (int y = 0; y < 300; y++) {
			auto neg_x  = x == 0 ? 0 : sum_grid[x-1][y];
			auto neg_y  = y == 0 ? 0 : sum_grid[x][y-1];
			auto neg_xy = (x == 0 || y == 0) ? 0 : sum_grid[x-1][y-1];
			// I(x, y) = i(x, y) + I(x, y-1) + I(x - 1, y) - I(x - 1, y - 1)
			sum_grid[x][y] = power_level(x+1, y+1, serial) + neg_x + neg_y - neg_xy;
		}
	}
	int max3_sum = int.min;
	ulong max3_x;
	ulong max3_y;
	int max_sum = int.min;
	ulong max_x;
	ulong max_y;
	ulong max_n;
	for (ulong x = 0; x < 300; x++) {
		for (ulong y = 0; y < 300; y++) {
			for (ulong n = 0; n < 300 - max(x, y); n++) {
				auto sum = sum_grid[x+n][y+n] - sum_grid[x][y+n] - sum_grid[x+n][y] + sum_grid[x][y];
				if (n == 3 && sum > max3_sum) {
					max3_sum = sum;
					max3_x = x+2; // Should probably work out why a +2 is necessary here..
					max3_y = y+2;
				}
				if (sum > max_sum) {
					max_sum = sum;
					max_x = x+2;
					max_y = y+2;
					max_n = n;
				}
			}
		}
	}
	writeln("Max grid coord: ", max3_x, ",", max3_y);
	writeln("Max grid coord with size: ", max_x, ",", max_y, ",", max_n);
}

unittest
{
	assert(power_level(  3,   5,  8) ==  4);
	assert(power_level(122,  79, 57) == -5);
	assert(power_level(217, 196, 39) ==  0);
	assert(power_level(101, 153, 71) ==  4);
}
