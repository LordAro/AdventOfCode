#include <iostream>
#include <fstream>
#include <sstream>
#include <map>
#include <vector>
#include <algorithm>

using graph_t = std::map<std::string, std::map<std::string, int>>;

int max_happiness(const graph_t &graph)
{
	std::vector<std::string> keys;
	for (const auto &it : graph) {
		keys.push_back(it.first);
	}

	int max = 0;
	do {
		int total = 0;
		for (int i = 0; i < keys.size() - 1; i++) {
			total += graph.at(keys[i]).at(keys[i + 1]);
			total += graph.at(keys[i + 1]).at(keys[i]);
		}
		// handle wrap round
		total += graph.at(keys.front()).at(keys.back());
		total += graph.at(keys.back()).at(keys.front());

		max = std::max(max, total);
	} while (std::next_permutation(keys.begin(), keys.end()));
	return max;
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

	graph_t graph;

	std::string line;
	while (std::getline(input, line)) {
		std::istringstream stream(line);
		std::string name1, dummy, change, name2;
		int happiness;
		stream >> name1 >> dummy >> change >> happiness >>
			dummy >> dummy >> dummy >> dummy >> dummy >> dummy >> name2;

		bool is_gain = change == "gain";
		if (!is_gain) happiness *= -1;

		// Remove '.'
		name2 = name2.substr(0, name2.size() - 1);

		graph[name1][name2] = happiness;
	}

	int max = max_happiness(graph);
	std::cout << "Maximum happiness: " << max << "\n";

	std::string me = "Me";
	for (const auto &it : graph) {
		graph[me][it.first] = 0;
		graph[it.first][me] = 0;
	}

	max = max_happiness(graph);
	std::cout << "Maximum happiness including yourself: " << max << "\n";
	return 0;
}
