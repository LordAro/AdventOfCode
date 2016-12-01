#include <iostream>
#include <fstream>
#include <sstream>
#include <map>
#include <set>
#include <vector>
#include <iomanip>
#include <algorithm>
#include <climits>

using graph_t = std::vector<std::vector<int>>;

std::pair<int, int> get_path(const graph_t &graph)
{
	std::vector<int> vertices(graph.size());
	std::iota(vertices.begin(), vertices.end(), 0);

	int min = INT_MAX;
	int max = 0;
	do {
		int total = 0;
		for (size_t i = 0; i < vertices.size() - 1; i++) {
			total += graph[vertices[i]][vertices[i + 1]];
		}
		min = std::min(min, total);
		max = std::max(max, total);

	} while (std::next_permutation(vertices.begin(), vertices.end()));
	return std::make_pair(min, max);
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

	std::map<std::pair<std::string, std::string>, int> edges;
	std::set<std::string> locations;

	std::string line;
	while (std::getline(input, line)) {
		std::string location1, through, location2, equals;
		int distance;
		std::istringstream stream(line);
		stream >> location1 >> through >> location2 >> equals >> distance;

		edges[std::make_pair(location1, location2)] = distance;
		locations.insert(location1);
		locations.insert(location2);
	}

	graph_t graph(locations.size(), std::vector<int>(locations.size()));

	for (const auto &edge : edges) {
		int loc1 = std::distance(locations.begin(), locations.find(edge.first.first));
		int loc2 = std::distance(locations.begin(), locations.find(edge.first.second));

		graph[loc1][loc2] = edge.second;
		graph[loc2][loc2] = 0;
		graph[loc1][loc1] = 0;
		graph[loc2][loc1] = edge.second;
	}

	for (const auto &row : graph) {
		for (const auto &d : row) {
			std::cout << std::setw(4) << d;
		}
		std::cout << "\n";
	}
	int min, max;
	std::tie(min, max) = get_path(graph);
	std::cout << "Shortest path: " << min << "\n";
	std::cout << "Longest path: " << max << "\n";

	return 0;
}
