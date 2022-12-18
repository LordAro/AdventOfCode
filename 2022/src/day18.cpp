#include <array>
#include <fstream>
#include <iostream>
#include <set>
#include <vector>

struct Coord3D {
	int x, y, z;

	bool operator<(const Coord3D &other) const
	{
		if (this->z != other.z) return this->z < other.z;
		if (this->y != other.y) return this->y < other.y;
		return this->x < other.x;
	}

	std::array<Coord3D, 6> get_adjacent_coords() const
	{
		std::array<Coord3D, 6> neighbours{
			Coord3D{this->x - 1, this->y, this->z},
			Coord3D{this->x + 1, this->y, this->z},
			Coord3D{this->x, this->y - 1, this->z},
			Coord3D{this->x, this->y + 1, this->z},
			Coord3D{this->x, this->y, this->z - 1},
			Coord3D{this->x, this->y, this->z + 1},
		};
		return neighbours;
	}
};

std::istream &operator>>(std::istream &is, Coord3D &coord)
{
	char c;
	is >> coord.x >> c >> coord.y >> c >> coord.z;
	return is;
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

	std::set<Coord3D> coords;

	for (Coord3D coord; input >> coord; ) {
		coords.insert(coord);
	}

	int total_free_spaces = 0;

	std::set<Coord3D> air_spaces;

	for (const auto &coord : coords) {
		int free_spaces = 0;
		for (const auto &neighbour : coord.get_adjacent_coords()) {
			if (coords.find(neighbour) == coords.end()) {
				air_spaces.insert(neighbour);
				free_spaces++;
			}
		}
		total_free_spaces += free_spaces;
	}

	std::cout << "Total surface area: " << total_free_spaces << '\n';

	// Additional loop to add neighbours of airspaces, for a cheap way of getting the airspace "diagonals"
	// In theory all external airspaces should now be a contiguous area
	std::set<Coord3D> diagonal_airspaces;
	for (const auto &coord : air_spaces) {
		for (const auto &neighbour : coord.get_adjacent_coords()) {
			if (coords.find(neighbour) == coords.end()) { // if non solid, add
				diagonal_airspaces.insert(neighbour); // don't add to the same set, or we'll be recursing infinitely
			}
		}
	}
	air_spaces.merge(diagonal_airspaces);

	// Floodfill to find all exposed airspaces
	std::set<Coord3D> external_airspaces;
	std::vector<Coord3D> to_search;
	to_search.push_back(*air_spaces.begin()); // In theory, the first air space could be inside... but this is good enough for our purposes

	while (!to_search.empty()) {
		auto next = to_search.back();
		to_search.pop_back();

		auto [_, inserted] = external_airspaces.insert(next);
		if (!inserted) {
			// already added
			continue;
		}

		for (const auto neighbour : next.get_adjacent_coords()) {
			if (air_spaces.find(neighbour) != air_spaces.end()) {
				to_search.push_back(neighbour);
			}
		}
	}

	int total_exposed_spaces = 0;
	for (const auto &coord : coords) {
		int exposed_airspaces = 0;
		for (const auto &neighbour : coord.get_adjacent_coords()) {
			if (external_airspaces.find(neighbour) != external_airspaces.end()) {
				exposed_airspaces++;
			}
		}
		total_exposed_spaces += exposed_airspaces;
	}
	std::cout << "Total external surface area: " << total_exposed_spaces << '\n';
}
