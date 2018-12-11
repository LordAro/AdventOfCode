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

	int[300][300] grid;
	for (int x = 0; x < 300; x++) {
		for (int y = 0; y < 300; y++) {
			grid[x][y] = power_level(x+1, y+1, serial);
		}
	}
	int max3_sum = int.min;
	int max3_x;
	int max3_y;
	int max_sum = int.min;
	int max_x;
	int max_y;
	int max_n;
	for (int x = 0; x < 300; x++) {
		for (int y = 0; y < 300; y++) {
			for (int n = 0; n < 300 - max(x, y); n++) {
				int sum = 0;
				for (int i = 0; i < n; i++) {
					for (int j = 0; j < n; j++) {
						sum += grid[x+i][y+j];
					}
				}
				if (n == 3 && sum > max3_sum) {
					max3_sum = sum;
					max3_x = x+1;
					max3_y = y+1;
				}
				if (sum > max_sum) {
					max_sum = sum;
					max_x = x+1;
					max_y = y+1;
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
