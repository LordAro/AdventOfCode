#include <array>
#include <fstream>
#include <iostream>
#include <unordered_set>

struct Coord {
	int x;
	int y;
};

bool operator==(const Coord &a, const Coord &b)
{
	return a.x == b.x && a.y == b.y;
}

// for std::unordered_set
struct CoordHash
{
	std::size_t operator()(Coord const &a) const
	{
		return (a.x << 16) + a.y; // much hack.
	}
};

Coord move_head(const Coord &old_pos, char dir)
{
	switch (dir) {
		case 'U':
			return {old_pos.x, old_pos.y + 1};
		case 'D':
			return {old_pos.x, old_pos.y - 1};
		case 'L':
			return {old_pos.x - 1, old_pos.y};
		case 'R':
			return {old_pos.x + 1, old_pos.y};
		default:
			__builtin_unreachable();
	}
}

Coord move_tail(const Coord &old_pos, const Coord &head_pos)
{
	int x_diff = head_pos.x - old_pos.x;
	int y_diff = head_pos.y - old_pos.y;

	const int x_sign = (x_diff > 0) - (x_diff < 0);
	const int y_sign = (y_diff > 0) - (y_diff < 0);
	if (x_diff == 2 || x_diff == -2 || y_diff == 2 || y_diff == -2) {
		// only move diagonally if there's a big difference
		// i.e. don't reduce difference of 1
		if (x_diff == 2 || x_diff == -2) {
			x_diff -= x_sign;
		}
		if (y_diff == 2 || y_diff == -2) {
			y_diff -= y_sign;
		}
	} else {
		// round to zero
		x_diff -= x_sign;
		y_diff -= y_sign;
	}

	return {old_pos.x + x_diff, old_pos.y + y_diff};
}

int main(int argc, char **argv)
{
	if (argc != 2) {
		std::cerr << "Incorrect number of arguments provided\n";
		return 1;
	}
	std::fstream input(argv[1]);
	if (!input) {
		std::cerr << "Could not open input file\n";
		return 1;
	}

	std::array<Coord, 10> longtail{};
	std::unordered_set<Coord, CoordHash> tail_positions;
	tail_positions.insert(longtail[1]); // second element behaves identically to the separate head/tail from part1
	std::unordered_set<Coord, CoordHash> longtail_positions;
	longtail_positions.insert(longtail.back());

	char dir;
	int n;
	while (input >> dir >> n) {
		for (int i = 0; i < n; i++) {
			longtail[0] = move_head(longtail[0], dir);
			for (size_t j = 1; j < longtail.size(); j++) {
				longtail[j] = move_tail(longtail[j], longtail[j - 1]);
			}
			tail_positions.insert(longtail[1]);
			longtail_positions.insert(longtail.back());
		}
	}

	std::cout << "Number of positions visited by tail: " << tail_positions.size() << '\n';
	std::cout << "Number of positions visited by long rope tail: " << longtail_positions.size() << '\n';
}
