#include <fstream>
#include <functional>
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

std::ostream &operator<<(std::ostream &os, FacingDir fd)
{
	switch (fd) {
		case FacingDir::East:
			os << '>';
			break;
		case FacingDir::South:
			os << 'v';
			break;
		case FacingDir::West:
			os << '<';
			break;
		case FacingDir::North:
			os << '^';
			break;
	}
	return os;
}

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

constexpr int EDGE_LEN = 50;

int get_face(Coord pos)
{
	if (EDGE_LEN <= pos.x && pos.x < 2*EDGE_LEN && 0 <= pos.y && pos.y < EDGE_LEN) {
		return 0; // A
	} else if (2*EDGE_LEN <= pos.x && pos.x < 3*EDGE_LEN && 0 <= pos.y && pos.y < EDGE_LEN) {
		return 1; // B
	} else if (EDGE_LEN <= pos.x && pos.x < 2*EDGE_LEN && EDGE_LEN <= pos.y && pos.y < 2*EDGE_LEN) {
		return 2; // C
	} else if (0 <= pos.x && pos.x < EDGE_LEN && 2*EDGE_LEN <= pos.y && pos.y < 3*EDGE_LEN) {
		return 3; // D
	} else if (EDGE_LEN <= pos.x && pos.x < 2*EDGE_LEN && 2*EDGE_LEN <= pos.y && pos.y < 3*EDGE_LEN) {
		return 4; // E
	} else if (0 <= pos.x && pos.x < EDGE_LEN && 3*EDGE_LEN <= pos.y && pos.y < 4*EDGE_LEN) {
		return 5; // F
	} else {
		__builtin_unreachable();
	}
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

	// determine position on face
	Coord relative_pos{pos.x % EDGE_LEN, pos.y % EDGE_LEN};

	// determine which face
	int face = get_face(pos);

	// map coord to corresponding face
	std::map<std::pair<int, FacingDir>, std::function<std::pair<Coord, FacingDir>(Coord)>> face_mappings {
		// A^ => F>
		{{0, FacingDir::North}, [](Coord rp) { return std::make_pair(Coord{0, 3*EDGE_LEN + rp.x}, FacingDir::East); } },
		// A< => D>
		{{0, FacingDir::West}, [](Coord rp) { return std::make_pair(Coord{0, 2*EDGE_LEN + (EDGE_LEN - rp.x - 1)}, FacingDir::East); } },

		// B^ => F^
		{{1, FacingDir::North}, [](Coord rp) { return std::make_pair(Coord{0 + rp.x, 4*EDGE_LEN - 1}, FacingDir::North); } },
		// B> => E<
		{{1, FacingDir::East}, [](Coord rp) { return std::make_pair(Coord{2*EDGE_LEN - 1, 2*EDGE_LEN + (EDGE_LEN - rp.x - 1)}, FacingDir::West); } },
		// Bv => C<
		{{1, FacingDir::South}, [](Coord rp) { return std::make_pair(Coord{2*EDGE_LEN - 1, EDGE_LEN + rp.x}, FacingDir::West); } },

		// C> => B^
		{{2, FacingDir::East}, [](Coord rp) { return std::make_pair(Coord{2*EDGE_LEN + rp.y, EDGE_LEN - 1}, FacingDir::North); } },
		// C< => Dv
		{{2, FacingDir::West}, [](Coord rp) { return std::make_pair(Coord{0 + rp.y, 3*EDGE_LEN}, FacingDir::South); } },

		// D^ => C>
		{{3, FacingDir::North}, [](Coord rp) { return std::make_pair(Coord{EDGE_LEN, EDGE_LEN + rp.x}, FacingDir::East); } },
		// D< => A>
		{{3, FacingDir::West}, [](Coord rp) { return std::make_pair(Coord{EDGE_LEN, 0 + (50 - rp.y - 1)}, FacingDir::East); } },

		// E> => B<
		{{4, FacingDir::East}, [](Coord rp) { return std::make_pair(Coord{3*EDGE_LEN - 1, 0 + (EDGE_LEN - rp.y - 1)}, FacingDir::West); } },
		// Ev => F<
		{{4, FacingDir::South}, [](Coord rp) { return std::make_pair(Coord{EDGE_LEN - 1, 3*EDGE_LEN + rp.y}, FacingDir::West); } },

		// F> => E^
		{{5, FacingDir::East}, [](Coord rp) { return std::make_pair(Coord{EDGE_LEN + rp.y, 3*EDGE_LEN - 1}, FacingDir::North); } },
		// Fv => Bv
		{{5, FacingDir::South}, [](Coord rp) { return std::make_pair(Coord{2*EDGE_LEN + rp.y, 0}, FacingDir::South); } },
		// F< => Av
		{{5, FacingDir::West}, [](Coord rp) { return std::make_pair(Coord{EDGE_LEN + rp.y, 0}, FacingDir::South); } },
	};

	auto face_map = face_mappings.at({face, fd});
	std::pair<Coord, FacingDir> new_pointdir = face_map(relative_pos);
	std::cout << pos << ' ' << fd << " => " << new_pointdir.first << ' ' << new_pointdir.second << '\n';
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
