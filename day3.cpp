#include <iostream>
#include <fstream>
#include <vector>
#include <algorithm>

struct Pos {
	int x;
	int y;
};

bool operator==(const Pos &a, const Pos &b)
{
	return a.x == b.x && a.y == b.y;
}

bool is_in(const std::vector<Pos> &vec, const Pos &x)
{
	return std::find(vec.begin(), vec.end(), x) != vec.end();
}

int main()
{
	std::fstream input("day3.input");
	if (!input.is_open()) {
		std::cout << "Could not open input file\n";
		return 1;
	}

	Pos santa_pos{0, 0};
	Pos robo_pos{0, 0};
	std::vector<Pos> visited = {santa_pos};

	int move_num = 0;

	char move;
	while (input >> move) {
		auto &cur_pos = move_num % 2 == 0 ? santa_pos : robo_pos;
		switch (move) {
			case '^': cur_pos.y++; break;
			case 'v': cur_pos.y--; break;
			case '>': cur_pos.x++; break;
			case '<': cur_pos.x--; break;
		}

		if (!is_in(visited, cur_pos)) {
			visited.push_back(cur_pos);
		}
		move_num++;
	}

	std::cout << "Houses that have received at least one present: " << visited.size() << "\n";

	return 0;
}