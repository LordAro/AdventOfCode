#include <iostream>
#include <fstream>
#include <algorithm>

bool contains_voxels(const std::string &line, int num)
{
	auto is_voxel = [](char c){return c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u';};
	return std::count_if(line.begin(), line.end(), is_voxel) >= num;
}

bool has_two_of_same(const std::string &line, int gap)
{
	for (int i = 0; i < line.size() - (gap + 1); i++) {
		if (line[i] == line[i + (gap + 1)]) return true;
	}
	return false;
}

bool contains_any_of(const std::string &line, const std::initializer_list<std::string> &list)
{
	for (const auto &item : list) {
		if (line.find(item) != std::string::npos) return true;
	}
	return false;
}

bool contains_pair_of_pair(const std::string &line)
{
	for (int i = 0; i < line.size() - 3; i++) {
		std::string pair = line.substr(i, 2);
		std::string remaining = line.substr(i+2, line.size());
		if (remaining.find(pair) != std::string::npos) return true;
	}
	return false;
}

int main()
{
	std::fstream input("day5.input");
	if (!input) {
		std::cout << "Could not open input file\n";
		return 1;
	}

	int count1 = 0;
	int count2 = 0;

	std::string line;
	while (input >> line) {
		bool nice1 = contains_voxels(line, 3);
		nice1 = nice1 && has_two_of_same(line, 0);
		nice1 = nice1 && !contains_any_of(line, {"ab", "cd", "pq", "xy"});
		if (nice1) count1++;

		bool nice2 = contains_pair_of_pair(line);
		nice2 = nice2 && has_two_of_same(line, 1);
		if (nice2) count2++;
	}

	std::cout << "Nice strings: " << count1 << "\n";
	std::cout << "With new rules: " << count2 << "\n";

	return 0;
}
