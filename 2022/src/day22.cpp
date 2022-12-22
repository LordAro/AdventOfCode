#include <fstream>
#include <iostream>
#include <map>
#include <variant>
#include <vector>

#include <sstream>

struct Coord {
	int x, y;

	bool operator<(const Coord &other) const
	{
		if (this->y != other.y) return this->y < other.y;
		return this->x < other.x;
	}

	Coord operator+(const Coord &other) const
	{
		return {this->x + other.x, this->y + other.y};
	}
};

std::ostream &operator<<(std::ostream &os, const Coord &c)
{
	os << c.x << ',' << c.y;
	return os;
}

enum class FacingDir {
	North,
	East,
	South,
	West,
};

enum class TurnDir {
	Left = -1,
	Right = 1,
};

FacingDir operator+(FacingDir fd, TurnDir td)
{
	return (FacingDir)(((int)fd + (int)td + 4) % 4); // +4 to ensure positive
}

using Instruction = std::variant<int, TurnDir>;
using Map = std::map<Coord, bool>;

// debugging
std::ostream &operator<<(std::ostream &os, const TurnDir td)
{
	os << ((td == TurnDir::Left) ? 'L' : 'R');
	return os;
}

std::ostream &operator<<(std::ostream &os, const Instruction &ins)
{
	if (std::holds_alternative<int>(ins)) {
		os << std::get<int>(ins);
	} else {
		os << std::get<TurnDir>(ins);
	}
	return os;
}

std::pair<Map, std::vector<Instruction>> parse_input(std::istream &is)
{
	Map map;
	int y = 0;
	for (std::string line; std::getline(is, line) && !line.empty(); y++) {
		for (int x = 0; x < (int)line.size(); x++) {
			if (line[x] == '.' || line[x] == '#') {
				map.try_emplace({x, y}, line[x] == '#');
			}
		}
	}

	std::vector<Instruction> instrs;
	while (true) {
		if (is.peek() == 'L' || is.peek() == 'R') {
			char t = is.get();
			instrs.push_back(t == 'L' ? TurnDir::Left : TurnDir::Right);
		} else {
			int v;
			// will trigger EOF
			is >> v;
			if (!is.good()) break;
			instrs.push_back(v);
		}
	}

	return {map, instrs};
}

Coord try_move(const Map &map, Coord pos, FacingDir fd)
{
	Coord move_dir{};
	switch (fd) {
		case FacingDir::North:
			move_dir = {0, -1};
			break;
		case FacingDir::East:
			move_dir = {1, 0};
			break;
		case FacingDir::South:
			move_dir = {0, 1};
			break;
		case FacingDir::West:
			move_dir = {-1, 0};
			break;
	}
	Coord new_pos = pos + move_dir;
	auto map_data = map.find(new_pos);
	if (map_data == map.end()) {
		// wrap around. Work out actual new coord
		switch (fd) {
			case FacingDir::North:
				for (const auto &kv : map) {
					if (kv.first.x == pos.x) {
						new_pos.y = std::max(new_pos.y, kv.first.y);
					}
				}
				break;
			case FacingDir::East:
				for (const auto &kv : map) {
					if (kv.first.y == pos.y) {
						new_pos.x = std::min(new_pos.x, kv.first.x);
					}
				}
				break;
			case FacingDir::South:
				for (const auto &kv : map) {
					if (kv.first.x == pos.x) {
						new_pos.y = std::min(new_pos.y, kv.first.y);
					}
				}
				break;
			case FacingDir::West:
				for (const auto &kv : map) {
					if (kv.first.y == pos.y) {
						new_pos.x = std::max(new_pos.x, kv.first.x);
					}
				}
				break;
		}
		map_data = map.find(new_pos);
	}

	if (map_data->second) { // '#'
		return pos; // no movement
	} else {
		return new_pos;
	}
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

	std::string example_input =
"        ...#\n"
"        .#..\n"
"        #...\n"
"        ....\n"
"...#.......#\n"
"........#...\n"
"..#....#....\n"
"..........#.\n"
"        ...#....\n"
"        .....#..\n"
"        .#......\n"
"        ......#.\n"
"\n"
"10R5L5R10L4R5L5\n";
	std::stringstream ex_input(example_input);

	const auto &[map, instructions] = parse_input(input);

//	for (const auto &i : instructions) std::cout << i << ' ';
//	std::cout << '\n';

	Coord pos = map.begin()->first; // map ordering means that "top left" coord is first
	FacingDir dir = FacingDir::East;
	for (const auto &ins : instructions) {
		if (std::holds_alternative<TurnDir>(ins)) {
			dir = dir + std::get<TurnDir>(ins);
		} else {
			for (int i = 0; i < std::get<int>(ins); i++) {
				pos = try_move(map, pos, dir);
			}
		}
	}

	int dir_score = 0;
	switch (dir) {
		case FacingDir::North:
			dir_score = 3;
			break;
		case FacingDir::East:
			dir_score = 0;
			break;
		case FacingDir::South:
			dir_score = 1;
			break;
		case FacingDir::West:
			dir_score = 2;
			break;
	}
	int password = 1000 * (pos.y + 1) + 4 * (pos.x + 1) + dir_score;
	std::cout << "Password: " << password << '\n';
}
