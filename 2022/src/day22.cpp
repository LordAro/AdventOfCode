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
	East,
	South,
	West,
	North,
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

std::pair<Coord, FacingDir> basic_wrap(const Map &map, Coord pos, FacingDir fd)
{
	Coord new_pos = pos;
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
	return {new_pos, fd}; // no change in direction
}

// My cube:
//  AB
//  C
// DE
// F
//
// A 50 <= x < 100, 0 <= y < 50
// B 100 <= x < 150, 0 <= y < 50
// C 50 <= x < 100, 50 <= y < 100
// D 0 <= x < 50, 100 <= y < 150
// E 50 <= x < 100, 100 <= y < 150
// F 0 <= x < 50, 150 <= y < 200
//
// A^ => F>
// A< => D>
// B^ => F^
// B> => E<
// Bv => C<
// C< => Dv
// C> => B^
// D^ => C>
// D< => A>
// E> => B<
// Ev => F<
// F> => E^
// Fv => Bv
// F< => Av
std::pair<Coord, FacingDir> cube_wrap(const Map &map, Coord pos, FacingDir fd)
{
	(void)map;
	const int EDGE_LEN = 50;
	// determine position on face
	Coord relative_pos{pos.x % EDGE_LEN, pos.y % EDGE_LEN};

	std::pair<Coord, FacingDir> new_pointdir;
	// determine which face
	// map coord to corresponding face
	if (EDGE_LEN <= pos.x && pos.x < 2*EDGE_LEN && 0 <= pos.y && pos.y < EDGE_LEN) {
		// A
		switch (fd) {
			case FacingDir::North: // => F>
				new_pointdir = {{0, 3*EDGE_LEN + relative_pos.x}, FacingDir::East};
				break;
			case FacingDir::West: // => D>
				new_pointdir = {{0, 2*EDGE_LEN + (EDGE_LEN - relative_pos.x - 1)}, FacingDir::East};
				break;
			default:
				__builtin_unreachable();
		}
	} else if (2*EDGE_LEN <= pos.x && pos.x < 3*EDGE_LEN && 0 <= pos.y && pos.y < EDGE_LEN) {
		// B
		switch (fd) {
			case FacingDir::North: // => F^
				new_pointdir = {{0 + relative_pos.x, 4*EDGE_LEN - 1}, FacingDir::North};
				break;
			case FacingDir::East: // => E<
				new_pointdir = {{2*EDGE_LEN - 1, 2*EDGE_LEN + (EDGE_LEN - relative_pos.x - 1)}, FacingDir::West};
				break;
			case FacingDir::South: // => C<
				new_pointdir = {{2*EDGE_LEN - 1, EDGE_LEN + relative_pos.x}, FacingDir::West};
				break;
			default:
				__builtin_unreachable();
		}
	} else if (50 <= pos.x && pos.x < 100 && 50 <= pos.y && pos.y < 100) {
		// C
		switch (fd) {
			case FacingDir::East: // => B^
				new_pointdir = {{2*EDGE_LEN + relative_pos.y, EDGE_LEN - 1}, FacingDir::North};
				break;
			case FacingDir::West: // => Dv
				new_pointdir = {{0 + relative_pos.y, 3*EDGE_LEN}, FacingDir::South};
				break;
			default:
				__builtin_unreachable();
		}
	} else if (0 <= pos.x && pos.x < 50 && 100 <= pos.y && pos.y < 150) {
		// D
		switch (fd) {
			case FacingDir::North: // => C>
				new_pointdir = {{EDGE_LEN, EDGE_LEN + relative_pos.x}, FacingDir::East};
				break;
			case FacingDir::West: // => A>
				new_pointdir = {{EDGE_LEN, 0 + (50 - relative_pos.y - 1)}, FacingDir::East};
				break;
			default:
				__builtin_unreachable();
		}
	} else if (50 <= pos.x && pos.x < 100 && 100 <= pos.y && pos.y < 150) {
		// E
		switch (fd) {
			case FacingDir::East: // => B<
				new_pointdir = {{3*EDGE_LEN - 1, 0 + (EDGE_LEN - relative_pos.y - 1)}, FacingDir::West};
				break;
			case FacingDir::South: // => F<
				new_pointdir = {{EDGE_LEN - 1, 3*EDGE_LEN + relative_pos.y}, FacingDir::West};
				break;
			default:
				__builtin_unreachable();
		}
	} else if (0 <= pos.x && pos.x < 50 && 150 <= pos.y && pos.y < 200) {
		// F
		switch (fd) {
			case FacingDir::East: // => E^
				new_pointdir = {{EDGE_LEN + relative_pos.y, 3*EDGE_LEN - 1}, FacingDir::North};
				break;
			case FacingDir::South: // => Bv
				new_pointdir = {{2*EDGE_LEN + relative_pos.y, 0}, FacingDir::South};
				break;
			case FacingDir::West: // => Av
				new_pointdir = {{EDGE_LEN + relative_pos.y, 0}, FacingDir::South};
				break;
			default:
				__builtin_unreachable();
		}
	} else {
		__builtin_unreachable(); // hopefully.
	}
	return new_pointdir;
}

template <typename F>
std::pair<Coord, FacingDir> try_move(const Map &map, Coord pos, FacingDir fd, F mapping_func)
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
	FacingDir new_dir = fd;
	auto map_data = map.find(new_pos);
	if (map_data == map.end()) {
		auto [new_pos_, new_dir_] = mapping_func(map, pos, fd);
		new_pos = new_pos_;
		new_dir = new_dir_;
		map_data = map.find(new_pos);
	}

	if (map_data->second) { // '#'
		return {pos, fd}; // no movement
	} else {
		return {new_pos, new_dir};
	}
}

int get_password(Coord pos, FacingDir fd)
{
	// FacingDir is constructed such that the integer values match
	return 1000 * (pos.y + 1) + 4 * (pos.x + 1) + (int)fd;
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

	{
		Coord pos = map.begin()->first; // map ordering means that "top left" coord is first
		FacingDir dir = FacingDir::East;
		for (const auto &ins : instructions) {
			if (std::holds_alternative<TurnDir>(ins)) {
				dir = dir + std::get<TurnDir>(ins);
			} else {
				for (int i = 0; i < std::get<int>(ins); i++) {
					std::tie(pos, dir) = try_move(map, pos, dir, basic_wrap);
				}
			}
		}

		std::cout << "Password with basic wrapping: " << get_password(pos, dir) << '\n';
	}


	{
		Coord pos = map.begin()->first; // map ordering means that "top left" coord is first
		FacingDir dir = FacingDir::East;
		for (const auto &ins : instructions) {
			if (std::holds_alternative<TurnDir>(ins)) {
				dir = dir + std::get<TurnDir>(ins);
			} else {
				for (int i = 0; i < std::get<int>(ins); i++) {
					std::tie(pos, dir) = try_move(map, pos, dir, cube_wrap);
				}
			}
		}
		std::cout << "Password with cube wrapping: " << get_password(pos, dir) << '\n';
	}
}
