#include <cassert>
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

std::ostream &operator<<(std::ostream &os, const State &state)
{
	std::cout << "T: " << state.time
		<< " O: " << state.ore << " (" << state.ore_robots << ")"
		<< " C: " << state.clay << " (" << state.clay_robots << ")"
		<< " Ob: " << state.obsidian << " (" << state.obsidian_robots << ")"
		<< " G: " << state.geodes << " (" << state.geode_robots << ")";
	return os;
}

enum CreateOption {
	OreRobot,
	ClayRobot,
	ObsidianRobot,
	GeodeRobot,
	CREATE_OPT_LIMIT,
};

template <int TIME_LIMIT>
int get_maximum_geodes(const Blueprint &blueprint, const State &state)
{
	if (state.time > TIME_LIMIT) {
		// done.
		// Need to remove some geodes if we've gone past the end of the time limit
		// so that we get the number of geodes at TIME_LIMIT
		// Last action should always be to create more geode robots, which we wouldn't have had
		return state.geodes - ((state.time - TIME_LIMIT - 1) * (state.geode_robots - 1));
	}

	int max_geode_count = 0;

	for (int i = 0; i < CREATE_OPT_LIMIT; i++) {
		CreateOption co = (CreateOption)i;
		// can't always create obsidian/geode robots
		if (co == ObsidianRobot && state.clay_robots == 0) continue;
		if (co == GeodeRobot && state.obsidian_robots == 0) continue;

		State new_state = state;
		int turn_count = 0;
		switch (co) {
			case OreRobot: {
				new_state.ore_robots++;
				new_state.ore -= blueprint.ore_cost;

				int needed_ore = blueprint.ore_cost - state.ore; // TODO: This can be a sum!
				while (needed_ore > 0) {
					needed_ore -= state.ore_robots;
					turn_count++;
				}
//				std::cout << "FOO1 " << time_step << '\n';
				break;
			}
			case ClayRobot: {
				new_state.clay_robots++;
				new_state.ore -= blueprint.clay_cost;
				int needed_ore = blueprint.clay_cost - state.ore; // TODO: This can be a sum!
				while (needed_ore > 0) {
					needed_ore -= state.ore_robots;
					turn_count++;
				}
//				std::cout << "FOO2 " << time_step << '\n';
				break;
			}
			case ObsidianRobot: {
				new_state.obsidian_robots++;
				new_state.ore -= blueprint.obsidian_cost.first;
				new_state.clay -= blueprint.obsidian_cost.second;
				int needed_ore = blueprint.obsidian_cost.first - state.ore; // TODO: This can be a sum!
				int needed_clay = blueprint.obsidian_cost.second - state.clay;
				while (needed_ore > 0 || needed_clay > 0) {
					needed_ore -= state.ore_robots;
					needed_clay -= state.clay_robots;
					turn_count++;
				}
//				std::cout << "FOO3 " << time_step << '\n';
				break;
			}
			case GeodeRobot: {
				new_state.geode_robots++;
				new_state.ore -= blueprint.geode_cost.first;
				new_state.obsidian -= blueprint.geode_cost.second;
				int needed_ore = blueprint.geode_cost.first - state.ore; // TODO: This can be a sum!
				int needed_obsidian = blueprint.geode_cost.second - state.obsidian;
				while (needed_ore > 0 || needed_obsidian > 0) {
					needed_ore -= state.ore_robots;
					needed_obsidian -= state.obsidian_robots;
					turn_count++;
				}
//				std::cout << "FOO4 " << time_step << '\n';
				break;
			}
			case CREATE_OPT_LIMIT:
				__builtin_unreachable();
				break;
		}
		// If we've already got enough resources and don't need to wait, we still need to wait 1 minute for the robot factory to complete its last order
		int time_step = std::max(turn_count, 0) + 1;
		// Only allow the last action to be creating a geode robot (no other action can have any effect, and makes the resulting calculation easier)
		if (co != GeodeRobot && state.time + time_step > TIME_LIMIT) continue;

		// Happens after we decide what we can create
		// Uses old state number of robots as we haven't created the new robots yet!
		new_state.time += time_step;
		new_state.ore += state.ore_robots * time_step;
		new_state.clay += state.clay_robots * time_step;
		new_state.obsidian += state.obsidian_robots * time_step;
		new_state.geodes += state.geode_robots * time_step;

		int geode_count = get_maximum_geodes<TIME_LIMIT>(blueprint, new_state);
		if (geode_count > max_geode_count) {
			max_geode_count = geode_count;
		}
	}
	return max_geode_count;
}

template <int TIME_LIMIT>
int run_blueprint(const Blueprint &blueprint)
{
	State starting_state{};
	starting_state.time = 1;
	starting_state.ore_robots = 1;
	int maximum_geodes = get_maximum_geodes<TIME_LIMIT>(blueprint, starting_state);
	return maximum_geodes;
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
//		run_blueprint(example1);
//		run_blueprint(example2);
//	}

	std::vector<Blueprint> blueprints;
	for (std::string line; std::getline(input, line); ) {
		blueprints.emplace_back(line);
	}

	int total_quality_level = 0;
	for (const auto &blueprint : blueprints) {
		int max_geodes = run_blueprint<24>(blueprint);
		std::cout << "Blueprint " << blueprint.num << ": " << max_geodes << " geodes. Quality level: " << max_geodes * blueprint.num << '\n';
		total_quality_level += max_geodes * blueprint.num;
	}
	std::cout << "Total quality level of all blueprints after 24 minutes: " << total_quality_level << '\n';

	int geode_multiply = 1;
	for (size_t i = 0; i < 3; i++) {
		const auto &blueprint = blueprints[i];
		int max_geodes = run_blueprint<32>(blueprint);
		std::cout << "Blueprint " << blueprint.num << ": " << max_geodes << " geodes. Quality level: " << max_geodes * blueprint.num << '\n';
		geode_multiply *= max_geodes;
	}
	std::cout << "Total geode multiplication of first 3 blueprints after 32 minutes: " << geode_multiply << '\n';
}
