#include <algorithm>
#include <deque>
#include <fstream>
#include <iostream>
#include <vector>
#include <sstream>

struct Coord {
	int x;
	int y;
};

std::ostream &operator<<(std::ostream &output, const Coord &c)
{
	output << c.x << ',' << c.y;
	return output;
}

bool operator==(const Coord &a, const Coord &b)
{
	return a.x == b.x && a.y == b.y;
}

using Grid2D = std::vector<std::vector<int>>;

std::vector<Coord> possible_moves(const Grid2D &heightmap, Coord pos)
{
	std::vector<Coord> moves;
	int cur_height = heightmap[pos.y][pos.x];
	if (pos.y > 0) {
		int new_height = heightmap[pos.y - 1][pos.x];
		if (new_height == cur_height || new_height == cur_height + 1) {
			moves.push_back({pos.x, pos.y - 1});
		}
	}
	if (pos.y < (int)heightmap.size() - 1) {
		int new_height = heightmap[pos.y + 1][pos.x];
		if (new_height == cur_height || new_height == cur_height + 1) {
			moves.push_back({pos.x, pos.y + 1});
		}
	}
	if (pos.x > 0) {
		int new_height = heightmap[pos.y][pos.x - 1];
		if (new_height == cur_height || new_height == cur_height + 1) {
			moves.push_back({pos.x - 1, pos.y});
		}
	}
	if (pos.x < (int)heightmap[pos.y].size() - 1) {
		int new_height = heightmap[pos.y][pos.x + 1];
		if (new_height == cur_height || new_height == cur_height + 1) {
			moves.push_back({pos.x + 1, pos.y});
		}
	}
	return moves;
}

// bfs
//fn find_all_routes(key: &[u8]) -> Vec<String> {
//    let mut routes = Vec::new();
//    let mut queue = VecDeque::new();
//    queue.push_back(("".to_string(), (0, 0)));
//    while !queue.is_empty() {
//        let (route, pos) = queue.pop_front().unwrap();
//        let h = get_route_hash(key, &route);
//        // foo
//        if pos == (3, 3) {
//            routes.push(route);
//            continue;
//        }
//        if pos.1 != 0 && is_door_open(h.chars().next().unwrap()) {
//            // U
//            queue.push_back((route.clone() + "U", get_new_pos(pos, 'U')));
//        }
//        if pos.1 != 3 && is_door_open(h.chars().nth(1).unwrap()) {
//            // D
//            queue.push_back((route.clone() + "D", get_new_pos(pos, 'D')));
//        }
//        if pos.0 != 0 && is_door_open(h.chars().nth(2).unwrap()) {
//            // L
//            queue.push_back((route.clone() + "L", get_new_pos(pos, 'L')));
//        }
//        if pos.0 != 3 && is_door_open(h.chars().nth(3).unwrap()) {
//            // R
//            queue.push_back((route.clone() + "R", get_new_pos(pos, 'R')));
//        }
//    }
//    routes
//}

using Route = std::vector<Coord>;
// bfs
Route get_shortest_route(const Grid2D &heightmap, Coord start_pos, Coord end_pos)
{
	std::deque<std::pair<Route, Coord>> queue;
	queue.push_back({{}, start_pos});
	while (!queue.empty()) {
		auto [route, pos] = queue.front();
		queue.pop_front();

//		std::cout << "Route: ";
//		for (const auto &c : route) std::cout << c << ' ';
//		std::cout << '\n';
		if (pos == end_pos) {
			return route;
		}

		std::vector<Coord> moves = possible_moves(heightmap, pos);
		for (const auto move : moves) {
			if (std::find(route.begin(), route.end(), move) != route.end()) continue;
			Route r2 = route;
			r2.push_back(pos);
			queue.push_back({r2, move});
		}
	}
	return {};
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

//	std::stringstream input("Sabqponm\n" "abcryxxl\n" "accszExk\n" "acctuvwj\n" "abdefghi");

	Grid2D heightmap;
	Coord start_pos;
	Coord end_pos;

	std::string line;
	while(std::getline(input, line)) {
		std::vector<int> row;
		row.reserve(line.size());
		for (char c : line) {
			if (c == 'S') {
				start_pos = {(int)row.size(), (int)heightmap.size()};
				c = 'a';
			} else if (c == 'E') {
				end_pos = {(int)row.size(), (int)heightmap.size()};
				c = 'z';
			}
			row.push_back(c - 'a');
		}
		heightmap.push_back(row);
	}
	std::cout << start_pos << '\n';
	std::cout << end_pos << '\n';

	auto route = get_shortest_route(heightmap, start_pos, end_pos);
	for (const auto &p : route) std::cout << p << ' ';
	std::cout << '\n';
	std::cout << "Shortest route: " << route.size() << '\n';
}
