#include <iostream>
#include <fstream>
#include <vector>
#include <algorithm>

std::vector<int> GetFactors(int num)
{
	std::vector<int> facts;
	for (int i = 1; i * i < num; i++) {
		if (num % i == 0) {
			facts.push_back(i);
			facts.push_back(num / i);
		}
	}
	return facts;
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

	int target;
	input >> target;

	int p1_house = 0;
	int p2_house = 0;
	for (int house = 1; p1_house == 0 || p2_house == 0; house++) {
		auto factors = GetFactors(house);
		int p1_presents = std::accumulate(factors.begin(), factors.end(), 0,
				[](int a, int b){return a + 10 * b;});
		if (p1_house == 0 && p1_presents >= target) p1_house = house;

		int p2_presents = std::accumulate(factors.begin(), factors.end(), 0,
				[house](int a, int b){return a + ((house / b > 50) ? 0 : 11 * b);});
		if (p2_house == 0 && p2_presents >= target) p2_house = house;
	}
	std::cout << "House with ridiculous number of presents: " << p1_house << "\n";
	std::cout << "House with ridiculous number of presents with different delivery system: " << p2_house << "\n";



	return 0;
}
