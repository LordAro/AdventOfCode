#include <iostream>
#include <fstream>
#include <unordered_set>

struct Pos {
	int x;
	int y;
};

bool operator==(const Pos &a, const Pos &b)
{
	return a.x == b.x && a.y == b.y;
}

struct PosHash
{
    std::size_t operator()(const Pos &s) const noexcept
    {
        std::size_t h1 = std::hash<int>{}(s.x);
        std::size_t h2 = std::hash<int>{}(s.y);
        return h1 ^ (h2 << 1); // or use boost::hash_combine
    }
};

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

	std::unordered_set<Pos, PosHash> visited = {normal_pos};
	std::unordered_set<Pos, PosHash> robo_assisted_visited = {santa_pos};

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

		visited.insert(normal_pos);
		robo_assisted_visited.insert(cur_pos);
		move_num++;
	}

	std::cout << "Houses that have received at least one present: " << visited.size() << "\n";
	std::cout << "Houses that have received at least one present (when Santa is helped by Robo-Santa): "
	          << robo_assisted_visited.size() << "\n";

	return 0;
}
