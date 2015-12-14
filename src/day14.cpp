#include <iostream>
#include <fstream>
#include <regex>
#include <vector>

struct Reindeer {
	Reindeer(const std::string &name, int speed, int time, int rest_time)
		: name(name), speed(speed), time(time), rest_time(rest_time), points(0)
	{}

	std::string name;
	int speed;
	int time;
	int rest_time;
	int dist;
	int points;

	int distance_in(int secs) const
	{
		int distance = 0;
		int move_time = this->time + this->rest_time;
		while (secs >= move_time) {
			distance += this->speed * this->time;
			secs -= move_time;
		}
		distance += speed * std::min(this->time, secs);
		return distance;
	}
};

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

	std::regex base_regex(R"((\S+) can fly (\S+) km/s for (\S+) seconds, but then must rest for (\S+) seconds.)");
	std::smatch base_match;

	std::vector<Reindeer> reindeers;

	std::string line;
	while (std::getline(input, line)) {
		std::regex_match(line, base_match, base_regex);
		std::string name = base_match[1];
		int speed = std::stoi(base_match[2]);
		int time = std::stoi(base_match[3]);
		int rest = std::stoi(base_match[4]);

		reindeers.emplace_back(name, speed, time, rest);
	}

	int max = 0;
	for (int secs = 1; secs <= 2503; secs++) {
		max = 0;
		for (auto &deer : reindeers) {
			int moved = deer.distance_in(secs);
			deer.dist = moved;
			max = std::max(max, moved);
		}
		// To handle case of multiple deer at same distance.
		for (auto &deer : reindeers) {
			if (deer.dist == max) deer.points++;
		}
	}
	auto points_func = [](const Reindeer &a, const Reindeer &b){return a.points < b.points;};
	int max_points = std::max_element(reindeers.begin(), reindeers.end(), points_func)->points;

	std::cout << "Maximum distance travelled: " << max << "\n";
	std::cout << "Maximum points: " << max_points << "\n";

	return 0;
}
