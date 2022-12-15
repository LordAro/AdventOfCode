#include <algorithm>
#include <fstream>
#include <iostream>
#include <limits>
#include <set>
#include <vector>

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

bool operator<(const Coord &a, const Coord &b)
{
	if (a.y != b.y) return a.y < b.y;
	return a.x < b.x;
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

	std::set<Coord> border_coords;
	// plan:
	// get all coords just outside border (dist + 1)
	for (const auto &sensor : sensors) {
		int search_dist = scan_radius(sensor) + 1;
		for (int i = 0; i <= search_dist; i++) {
			Coord c1{sensor.coord.x - search_dist + i, sensor.coord.y + i};
			Coord c2{sensor.coord.x - search_dist + i, sensor.coord.y - i};
			Coord c3{sensor.coord.x + search_dist - i, sensor.coord.y + i};
			Coord c4{sensor.coord.x + search_dist - i, sensor.coord.y - i};
			if (c1.x >= revised_bounding_rect_bl.x && c1.x <= revised_bounding_rect_tr.x && c1.y >= revised_bounding_rect_bl.y && c1.y <= revised_bounding_rect_tr.y) {
				border_coords.insert(c1);
			}
			if (c2.x >= revised_bounding_rect_bl.x && c2.x <= revised_bounding_rect_tr.x && c2.y >= revised_bounding_rect_bl.y && c2.y <= revised_bounding_rect_tr.y) {
				border_coords.insert(c2);
			}
			if (c3.x >= revised_bounding_rect_bl.x && c3.x <= revised_bounding_rect_tr.x && c3.y >= revised_bounding_rect_bl.y && c3.y <= revised_bounding_rect_tr.y) {
				border_coords.insert(c3);
			}
			if (c4.x >= revised_bounding_rect_bl.x && c4.x <= revised_bounding_rect_tr.x && c4.y >= revised_bounding_rect_bl.y && c4.y <= revised_bounding_rect_tr.y) {
				border_coords.insert(c4);
			}
		}
	}

	// check all those coords to see if they're inside another sensor's search sphere
	// find the (hopefully) one that isn't
	Coord distress_beacon;
	for (const auto &border_coord : border_coords) {
		if (std::none_of(sensors.begin(), sensors.end(), [border_coord](const Sensor &sensor) {
				return manhattan_distance(sensor.coord, border_coord) <= scan_radius(sensor);
		})) {
			distress_beacon = border_coord;
			break;
		}
	}
	std::cout << "Tuning frequency of distress beacon (at " << distress_beacon << "): " << (int64_t)distress_beacon.x * 4'000'000 + distress_beacon.y << '\n';
}
