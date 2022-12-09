#include <array>
#include <fstream>
#include <iostream>
#include <set>

struct Coord {
	int x;
	int y;
};

bool operator==(const Coord a, const Coord b)
{
	return a.x == b.x && a.y == b.y;
}

// for std::set
bool operator<(const Coord a, const Coord b)
{
	if (a.x != b.x) return a.x < b.x;
	return a.y < b.y;
}

Coord move_head(const Coord old_pos, char dir)
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
			throw "Unknown direction";
	}
}

Coord move_tail(const Coord old_pos, const Coord head_pos)
{
	if (head_pos == old_pos) return old_pos;

	int x_diff = head_pos.x - old_pos.x;
	int y_diff = head_pos.y - old_pos.y;

	if (x_diff == 2 && y_diff == 2) {
		x_diff--;
		y_diff--;
	} else if (x_diff == -2 && y_diff == -2) {
		x_diff++;
		y_diff++;
	} else if (x_diff == 2 && y_diff == -2) {
		x_diff--;
		y_diff++;
	} else if (x_diff == -2 && y_diff == 2) {
		x_diff++;
		y_diff--;
	} else if (x_diff == 2) {
		x_diff--;
	} else if (x_diff == -2) {
		x_diff++;
	} else if (y_diff == 2) {
		y_diff--;
	} else if (y_diff == -2) {
		y_diff++;
	} else {
		// round to zero
		if (x_diff > 0) x_diff--;
		if (y_diff > 0) y_diff--;
		if (x_diff < 0) x_diff++;
		if (y_diff < 0) y_diff++;
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

	Coord head{0, 0};
	Coord tail{0, 0};

	std::set<Coord> tail_positions;
	tail_positions.insert(tail);

	std::array<Coord, 10> longtail{};
	std::set<Coord> longtail_positions;
	longtail_positions.insert(longtail.back());

	char dir;
	int n;
	while (input >> dir >> n) {
		for (int i = 0; i < n; i++) {
			head = move_head(head, dir);
			tail = move_tail(tail, head);
			tail_positions.insert(tail);

			longtail[0] = move_head(longtail[0], dir);
			for (size_t j = 1; j < longtail.size(); j++) {
				longtail[j] = move_tail(longtail[j], longtail[j - 1]);
			}
			longtail_positions.insert(longtail.back());
		}
	}

	std::cout << "Number of positions visited by tail: " << tail_positions.size() << '\n';
	std::cout << "Number of positions visited by long rope tail: " << longtail_positions.size() << '\n';
}
