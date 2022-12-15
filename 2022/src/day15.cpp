#include <algorithm>
#include <fstream>
#include <iostream>
#include <limits>
#include <numeric>
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

struct Interval {
	int start, end;
};

std::vector<Interval> merge_intervals(std::vector<Interval> intervals)
{
	std::sort(intervals.begin(), intervals.end(), [](const Interval &a, const Interval &b) { return a.start < b.start; });

	std::vector<Interval> merged;
	merged.push_back(intervals[0]);
	for (size_t i = 1; i < intervals.size(); i++) {
		Interval top = merged.back();
		if (top.end < intervals[i].start) {
			merged.push_back(intervals[i]);
		} else if (top.end < intervals[i].end) {
			top.end = intervals[i].end;
			merged.pop_back();
			merged.push_back(top);
		}
	}
	return merged;
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

	const int SCAN_Y = 2'000'000;
	std::vector<Interval> intervals;
	std::set<Coord> beacons_on_y;
	for (const auto &sensor : sensors) {
		int search_dist = scan_radius(sensor);
		// if search radius crosses line
		// otherwise, difference would be negative and screw things up
		if ((sensor.coord.y <= SCAN_Y && sensor.coord.y + search_dist >= SCAN_Y)
				|| (sensor.coord.y >= SCAN_Y && sensor.coord.y - search_dist <= SCAN_Y)) {
			int intersect_radius = std::abs(sensor.coord.y - SCAN_Y);
			int difference = search_dist - intersect_radius;

			intervals.push_back({sensor.coord.x - difference, sensor.coord.x + difference});
			if (sensor.closest_beacon.y == SCAN_Y) {
				beacons_on_y.insert(sensor.closest_beacon);
			}
		}
	}

	auto merged_intervals = merge_intervals(intervals);
	int total_x_in_shadow = std::accumulate(merged_intervals.begin(), merged_intervals.end(), 0,
			[](int sum, const Interval &interval) { return sum + interval.end - interval.start + 1; } // +1 as ranges are inclusive
	);
	std::cout << "Number of spaces at y=" << SCAN_Y << " that cannot be beacons: " << total_x_in_shadow - beacons_on_y.size() << '\n';

	Coord revised_bounding_rect_bl{std::max(bounding_rect_bl.x, 0), std::max(bounding_rect_bl.y, 0)};
	Coord revised_bounding_rect_tr{std::min(bounding_rect_tr.x, 4'000'000), std::min(bounding_rect_tr.y, 4'000'000)};

	// Greg's geometric solution. Very fancy.
	// each sensor casts 2 / direction lines and 2 \ direction lines from the edges of the diamond of its range.
	// the unique uncovered square must be in the middle of an intersection of these.
	// we assume that the uncovered square is not on the edge of the world, and so only need to consider 1 of the 2 in each pair of parallel lines.
	std::set<int> pos_lines;
	std::set<int> neg_lines;
	for (const auto &sensor : sensors) {
		const int search_dist = scan_radius(sensor) + 1;
		const Coord l{sensor.coord.x - search_dist, sensor.coord.y};
		pos_lines.insert(l.y - l.x); // '/'
		neg_lines.insert(l.y + l.x); // '\'
	}

	std::vector<int> double_pos_lines;
	std::vector<int> double_neg_lines;
	for (const auto &sensor : sensors) {
		const int search_dist = scan_radius(sensor) + 1;
		const Coord r{sensor.coord.x + search_dist, sensor.coord.y};
		if (pos_lines.find(r.y - r.x) != pos_lines.end()) {
			double_pos_lines.push_back(r.y - r.x); // '/'
		}
		if (neg_lines.find(r.y + r.x) != neg_lines.end()) {
			double_neg_lines.push_back(r.y + r.x); // '\'
		}
	}

	Coord distress_beacon{0, 0};
	for (const auto &pos : double_pos_lines) {
		for (const auto &neg : double_neg_lines) {
			const Coord crossing{(neg - pos) / 2, (neg + pos) / 2};
			if (crossing.x >= revised_bounding_rect_bl.x && crossing.x <= revised_bounding_rect_tr.x
					&& crossing.y >= revised_bounding_rect_bl.y && crossing.y <= revised_bounding_rect_tr.y) {
				distress_beacon = crossing;
				goto outer; // yay goto
			}
		}
	}
outer:
	std::cout << "Tuning frequency of distress beacon (at " << distress_beacon << "): " << (int64_t)distress_beacon.x * 4'000'000 + distress_beacon.y << '\n';
}
