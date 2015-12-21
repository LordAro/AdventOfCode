#include <iostream>
#include <fstream>
#include <vector>
#include <cmath>

struct Person {
	Person(int hp, int dam, int arm) : hitpoints(hp), damage(dam), armour(arm)
	{
	}

	int hitpoints;
	int damage;
	int armour;
};

struct Item {
	Item(int cost, int dam, int arm) : cost(cost), damage(dam), armour(arm)
	{
	}

	int cost;
	int damage;
	int armour;
};

bool operator==(const Item &a, const Item &b)
{
	return a.cost == b.cost && a.damage == b.damage && a.armour == b.armour;
}

bool PlayGame(Person boss, Person player)
{
	float p_damage = std::max(1, player.damage - boss.armour);
	float b_damage = std::max(1, boss.damage - player.armour);
	return std::ceil(player.hitpoints / b_damage) >= std::ceil(boss.hitpoints / p_damage);
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

	std::vector<Item> weapons; // must have a weapon
	weapons.emplace_back( 8, 4, 0);
	weapons.emplace_back(10, 5, 0);
	weapons.emplace_back(25, 6, 0);
	weapons.emplace_back(40, 7, 0);
	weapons.emplace_back(74, 8, 0);

	std::vector<Item> armour;
	armour.emplace_back(  0, 0, 0); // no item
	armour.emplace_back( 13, 0, 1);
	armour.emplace_back( 31, 0, 2);
	armour.emplace_back( 53, 0, 3);
	armour.emplace_back( 75, 0, 4);
	armour.emplace_back(102, 0, 5);

	std::vector<Item> rings;
	rings.emplace_back(  0, 0, 0); // no item
	rings.emplace_back( 25, 1, 0);
	rings.emplace_back( 50, 2, 0);
	rings.emplace_back(100, 3, 0);
	rings.emplace_back( 20, 0, 1);
	rings.emplace_back( 40, 0, 2);
	rings.emplace_back( 80, 0, 3);

	// Get boss stats
	std::string dummy;
	int b_hitpoints, b_damage, b_armour;
	input >> dummy >> dummy >> b_hitpoints;
	input >> dummy >> b_damage;
	input >> dummy >> b_armour;

	Person boss(b_hitpoints, b_damage, b_armour);

	int min_cost = 5000;
	int max_cost = 0;
	for (const auto &w : weapons) {
		for (const auto &a : armour) {
			for (const auto &r1 : rings) {
				for (const auto &r2 : rings) {
					if (r1 == r2) continue; // Can't have 2 of the same ring
					int cost = w.cost + a.cost + r1.cost + r2.cost;
					int p_damage = w.damage + a.damage + r1.damage + r2.damage;
					int p_armour = w.armour + a.armour + r1.armour + r2.armour;
					Person player(100, p_damage, p_armour);

					bool win = PlayGame(boss, player);
					if (win) {
						min_cost = std::min(min_cost, cost);
					} else {
						max_cost = std::max(max_cost, cost);
					}
				}
			}
		}
	}
	std::cout << "Minimal cost needed to win: " << min_cost << "\n";
	std::cout << "Maximal cost and still lose: " << max_cost << "\n";

	return 0;
}
