#include <fstream>
#include <iostream>
#include <map>
#include <vector>

struct Blueprint {
	Blueprint(const std::string &line);
	int num;
	int ore_cost;
	int clay_cost;
	std::pair<int, int> obsidian_cost;
	std::pair<int, int> geode_cost;
};

Blueprint::Blueprint(const std::string &line)
{
	int ret = sscanf(line.c_str(), "Blueprint %d: Each ore robot costs %d ore. Each clay robot costs %d ore. Each obsidian robot costs %d ore and %d clay. Each geode robot costs %d ore and %d obsidian.",
			&this->num, &this->ore_cost, &this->clay_cost, &this->obsidian_cost.first, &this->obsidian_cost.second, &this->geode_cost.first, &this->geode_cost.second);
	if (ret != 7) {
		throw "Could not parse input string";
	}
}

struct State {
	int time;
	int ore_robots;
	int clay_robots;
	int obsidian_robots;
	int geode_robots;
	int ore;
	int clay;
	int obsidian;
	int geodes;

	bool operator<(const State &other) const
	{
		if (this->geodes != other.geodes) return this->geodes < other.geodes;
		if (this->geode_robots != other.geode_robots) return this->geode_robots < other.geode_robots;
		if (this->obsidian != other.obsidian) return this->obsidian < other.obsidian;
		if (this->obsidian_robots != other.obsidian_robots) return this->obsidian_robots < other.obsidian_robots;
		if (this->clay != other.clay) return this->clay < other.clay;
		if (this->clay_robots != other.clay_robots) return this->clay_robots < other.clay_robots;
		if (this->ore != other.ore) return this->ore < other.ore;
		if (this->ore_robots != other.ore_robots) return this->ore_robots < other.ore_robots;
		return this->time < other.time;
	}
};

const int TIME_LIMIT = 24;

enum CreateOption {
	DoNothing,
	OreRobot,
	ClayRobot,
	ObsidianRobot,
	GeodeRobot,
};

using MemoiseType = std::map<std::pair<State, int>, int>;

int get_maximum_geodes(MemoiseType &memoising, const Blueprint &blueprint, const State &state, int time)
{
	if (time > TIME_LIMIT) {
		return state.geodes; // done.
	}

	auto memo = memoising.find({state, time});
	if (memo != memoising.end()) {
		return memo->second;
	}

	std::vector<CreateOption> create_options;
	create_options.push_back(DoNothing);
	if (state.ore >= blueprint.ore_cost) {
		// costs ore
		create_options.push_back(OreRobot);
	}
	if (state.ore >= blueprint.clay_cost) {
		// costs ore
		create_options.push_back(ClayRobot);
	}
	if (state.ore >= blueprint.obsidian_cost.first && state.clay >= blueprint.obsidian_cost.second) {
		// costs ore and clay
		create_options.push_back(ObsidianRobot);
	}
	if (state.ore >= blueprint.geode_cost.first && state.obsidian >= blueprint.geode_cost.second) {
		// costs ore and obsidian
		create_options.push_back(GeodeRobot);
	}

	int max_geode_count = 0;
	State new_state_max;
	// TODO: More than one thing at once?
	for (CreateOption co : create_options) {
		State new_state = state;
		switch (co) {
			case DoNothing:
				break;
			case OreRobot:
				new_state.ore_robots++;
				new_state.ore -= blueprint.ore_cost;
				break;
			case ClayRobot:
				new_state.clay_robots++;
				new_state.ore -= blueprint.clay_cost;
				break;
			case ObsidianRobot:
				new_state.obsidian_robots++;
				new_state.ore -= blueprint.obsidian_cost.first;
				new_state.clay -= blueprint.obsidian_cost.second;
				break;
			case GeodeRobot:
				new_state.geode_robots++;
				new_state.ore -= blueprint.geode_cost.first;
				new_state.obsidian -= blueprint.geode_cost.second;
				break;
		}

		// Happens after we decide what we can create
		// Uses old state number of robots as we haven't created the new robots yet!
		new_state.ore += state.ore_robots;
		new_state.clay += state.clay_robots;
		new_state.obsidian += state.obsidian_robots;
		new_state.geodes += state.geode_robots;

		int geode_count = get_maximum_geodes(memoising, blueprint, new_state, time + 1);
		if (geode_count > max_geode_count) {
			max_geode_count = geode_count;
			new_state_max = new_state;
		}
	}
	memoising[{new_state_max, time + 1}] = max_geode_count;
	return max_geode_count;
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

//	{
//		Blueprint example1("Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.");
//		Blueprint example2("Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.");
//
//		State starting_state{};
//		starting_state.ore_robots = 1;
//		MemoiseType memo;
//		int maximum_geodes = get_maximum_geodes(memo, example1, starting_state, 1);
//		int quality_level = example1.num * maximum_geodes;
//		std::cout << "Blueprint " << example1.num << ": " << maximum_geodes << " geodes. Quality level: " << quality_level << '\n';
//	}
	std::vector<Blueprint> blueprints;

	for (std::string line; std::getline(input, line); ) {
		blueprints.emplace_back(line);
	}

	int total_quality_level = 0;
	for (const auto &blueprint : blueprints) {
		State starting_state{};
		starting_state.ore_robots = 1;
		MemoiseType memo;
		int maximum_geodes = get_maximum_geodes(memo, blueprint, starting_state, 1);
		int quality_level = blueprint.num * maximum_geodes;
		std::cout << "Blueprint " << blueprint.num << ": " << maximum_geodes << " geodes. Quality level: " << quality_level << '\n';
		total_quality_level += quality_level;
	}
	std::cout << "Total quality level of all blueprints: " << total_quality_level << '\n';
}
