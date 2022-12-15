#include <algorithm>
#include <fstream>
#include <iostream>
#include <limits>
#include <set>
#include <vector>
#include <sstream>

struct Coord {
	int x;
	int y;
};

std::ostream &operator<<(std::ostream &os, const Coord &coord)
{
	os << coord.x << ',' << coord.y;
	return os;
}

std::istream &operator>>(std::istream &is, Coord &coord)
{
	is.ignore(2); // 'x='
	is >> coord.x;
	is.ignore(4); // ', y='
	is >> coord.y;
	return is;
}

bool operator==(const Coord &a, const Coord &b)
{
	return a.x == b.x && a.y == b.y;
}

bool operator!=(const Coord &a, const Coord &b)
{
	return !(a == b);
}

struct Sensor {
	Coord coord;
	Coord closest_beacon;
};

std::ostream &operator<<(std::ostream &os, const Sensor &sensor)
{
	os << sensor.coord << " => " << sensor.closest_beacon;
	return os;
}

std::istream &operator>>(std::istream &is, Sensor &sensor)
{
	std::string skip;
	is >> skip >> skip; // Sensor at
	is.ignore(1); // ' '
	is >> sensor.coord;
	is.ignore(1); // :
	is >> skip >> skip >> skip >> skip; // closest beacon is at
	is.ignore(1); // ' '
	is >> sensor.closest_beacon;
	return is;
}

int manhattan_distance(const Coord &a, const Coord &b)
{
	return std::abs(a.x - b.x) + std::abs(a.y - b.y);
}

int scan_radius(const Sensor &sensor)
{
	return manhattan_distance(sensor.coord, sensor.closest_beacon);
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
"Sensor at x=2, y=18: closest beacon is at x=-2, y=15\n"
"Sensor at x=9, y=16: closest beacon is at x=10, y=16\n"
"Sensor at x=13, y=2: closest beacon is at x=15, y=3\n"
"Sensor at x=12, y=14: closest beacon is at x=10, y=16\n"
"Sensor at x=10, y=20: closest beacon is at x=10, y=16\n"
"Sensor at x=14, y=17: closest beacon is at x=10, y=16\n"
"Sensor at x=8, y=7: closest beacon is at x=2, y=10\n"
"Sensor at x=2, y=0: closest beacon is at x=2, y=10\n"
"Sensor at x=0, y=11: closest beacon is at x=2, y=10\n"
"Sensor at x=20, y=14: closest beacon is at x=25, y=17\n"
"Sensor at x=17, y=20: closest beacon is at x=21, y=22\n"
"Sensor at x=16, y=7: closest beacon is at x=15, y=3\n"
"Sensor at x=14, y=3: closest beacon is at x=15, y=3\n"
"Sensor at x=20, y=1: closest beacon is at x=15, y=3\n"
;
	std::stringstream ex_input(example_input);

	Coord bounding_rect_bl = {std::numeric_limits<int>::max(), std::numeric_limits<int>::max()};
	Coord bounding_rect_tr = {std::numeric_limits<int>::min(), std::numeric_limits<int>::min()};

	std::vector<Sensor> sensors;
	for (Sensor sensor; input >> sensor; ) {
		int search_dist = scan_radius(sensor);
		bounding_rect_bl.x = std::min(bounding_rect_bl.x, sensor.coord.x - search_dist);
		bounding_rect_bl.y = std::min(bounding_rect_bl.y, sensor.coord.y - search_dist);

		bounding_rect_tr.x = std::max(bounding_rect_tr.x, sensor.coord.x + search_dist);
		bounding_rect_tr.y = std::max(bounding_rect_tr.y, sensor.coord.y + search_dist);
		sensors.push_back(sensor);
	}

	//const int SCAN_Y = 10;
	const int SCAN_Y = 2'000'000;
	int empty_spaces_count = 0;

	for (int scan_x = bounding_rect_bl.x; scan_x <= bounding_rect_tr.x; scan_x++) {
		Coord scan_coord = {scan_x, SCAN_Y};
		if (std::any_of(sensors.begin(), sensors.end(), [scan_coord](const Sensor &sensor) {
				return manhattan_distance(sensor.coord, scan_coord) <= scan_radius(sensor) && scan_coord != sensor.closest_beacon;
		})) {
			empty_spaces_count++;
		}
	}

	std::cout << "Number of spaces at y=" << SCAN_Y << " that cannot be beacons: " << empty_spaces_count << '\n';

	Coord revised_bounding_rect_bl = {std::max(bounding_rect_bl.x, 0), std::max(bounding_rect_bl.y, 0)};
	Coord revised_bounding_rect_tr = {std::min(bounding_rect_tr.x, 4'000'000), std::min(bounding_rect_tr.y, 4'000'000)};
	Coord distress_beacon;
	for (int scan_x = revised_bounding_rect_bl.x; scan_x <= revised_bounding_rect_tr.x; scan_x++) {
		if(scan_x % 4000 == 0) std::cout << scan_x << '\n';
		for (int scan_y = revised_bounding_rect_bl.y; scan_y <= revised_bounding_rect_tr.y; scan_y++) {
			Coord scan_coord = {scan_x, scan_y};
			if (std::none_of(sensors.begin(), sensors.end(), [scan_coord](const Sensor &sensor) {
					return manhattan_distance(sensor.coord, scan_coord) <= scan_radius(sensor) && scan_coord != sensor.closest_beacon;
			})) {
				distress_beacon = scan_coord;
				goto outer; // yay goto!
			}

		}
	}
outer:
	std::cout << "Tuning frequency of distress beacon (at " << distress_beacon << "): " << (int64_t)distress_beacon.x * 4'000'000 + distress_beacon.y << '\n';
}
