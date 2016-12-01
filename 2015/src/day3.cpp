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

	Pos normal_pos{0, 0};

	Pos santa_pos{0, 0};
	Pos robo_pos{0, 0};

	std::vector<Pos> visited = {normal_pos};
	std::vector<Pos> robo_assisted_visited = {santa_pos};

	int move_num = 0;

	char move;
	while (input >> move) {
		auto &cur_pos = move_num % 2 == 0 ? santa_pos : robo_pos;
		switch (move) {
			case '^': cur_pos.y++; normal_pos.y++; break;
			case 'v': cur_pos.y--; normal_pos.y--; break;
			case '>': cur_pos.x++; normal_pos.x++; break;
			case '<': cur_pos.x--; normal_pos.x--; break;
		}

		if (!is_in(visited, normal_pos)) {
			visited.push_back(normal_pos);
		}
		if (!is_in(robo_assisted_visited, cur_pos)) {
			robo_assisted_visited.push_back(cur_pos);
		}
		move_num++;
	}

	std::cout << "Houses that have received at least one present: " << visited.size() << "\n";
	std::cout << "Houses that have received at least one present (when Santa is helped by Robo-Santa): "
	          << robo_assisted_visited.size() << "\n";

	return 0;
}
