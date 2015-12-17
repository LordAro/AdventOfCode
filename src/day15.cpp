#include <iostream>
#include <fstream>
#include <regex>


struct Ingredient {
	Ingredient(const std::string &name, int cap, int dur, int flav, int tex, int cal)
		: name(name), capacity(cap), durability(dur), flavour(flav), texture(tex), calories(cal)
	{
	}

	std::string name;
	int capacity;
	int durability;
	int flavour;
	int texture;
	int calories;
};

std::vector<std::vector<int>> get_combinations(int sum, int len)
{
	if (len < 1) return {{}};
	if (len == 1) return {{sum}};

	std::vector<std::vector<int>> list;
	for (int i = 0; i <= sum; i++) {
		auto tmp = get_combinations(sum - i, len - 1);
		for (auto arr : tmp) {
			arr.push_back(i);
			list.push_back(arr);
		}
	}
	return list;
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

	std::regex base_regex(R"((\S+): capacity (\S+), durability (\S+), flavor (\S+), texture (\S+), calories (\S+))");
	std::smatch base_match;

	std::vector<Ingredient> items;

	std::string line;
	while (std::getline(input, line)) {
		std::regex_match(line, base_match, base_regex);
		std::string name = base_match[1];
		int cap = std::stoi(base_match[2]);
		int dur = std::stoi(base_match[3]);
		int fla = std::stoi(base_match[4]);
		int tex = std::stoi(base_match[5]);
		int cal = std::stoi(base_match[6]);

		items.emplace_back(name, cap, dur, fla, tex, cal);
	}

	int max_score = 0;
	std::vector<int> best_comb;

	// Part2
	int max_lowcal_score = 0;
	std::vector<int> best_lowcal_comb;

	auto combinations = get_combinations(100, items.size());
	for (const auto &comb : combinations) {
		int cap = 0;
		int dur = 0;
		int fla = 0;
		int tex = 0;
		int cal = 0;
		for (size_t i = 0; i < items.size(); i++) {
			cap += items[i].capacity   * comb[i];
			dur += items[i].durability * comb[i];
			fla += items[i].flavour    * comb[i];
			tex += items[i].texture    * comb[i];
			cal += items[i].calories   * comb[i];
		}
		cap = std::max(0, cap);
		dur = std::max(0, dur);
		fla = std::max(0, fla);
		tex = std::max(0, tex);
		// calories can't be negative
		int score = cap * dur * fla * tex;
		if (score > max_score) {
			max_score = score;
			best_comb = comb;
		}
		if (score > max_lowcal_score && cal == 500) {
			max_lowcal_score = score;
			best_lowcal_comb = comb;
		}

	}

	std::cout << "Maximum cookie score: " << max_score;
	std::cout << " with combination: ";
	for (const auto &it : best_comb) std::cout << it << " ";
	std::cout << "\n";

	// Part2
	std::cout << "Maximum low-cal cookie score: " << max_lowcal_score;
	std::cout << " with combination: ";
	for (const auto &it : best_lowcal_comb) std::cout << it << " ";
	std::cout << "\n";

	return 0;
}
