#include <iostream>
#include <fstream>
#include <vector>
#include <algorithm>
#include <map>

struct Person {
	Person(int hp, int dam, int arm, int man) : hp(hp), dam(dam), arm(arm), man(man)
	{
	}

	int hp;
	int dam;
	int arm;
	int man;

	bool IsAlive()
	{
		return this->hp > 0;
	}
};

enum class Spell {
	Missile,
	Drain,
	Shield,
	Poison,
	Recharge,
	First = Missile,
	Last = Recharge,
};

static const std::map<Spell, std::pair<std::string, int>> SPELL_MAP = {
	{Spell::Missile, {"Missile", 53}},
	{Spell::Drain, {"Drain", 73}},
	{Spell::Shield, {"Shield", 113}},
	{Spell::Poison, {"Poison", 173}},
	{Spell::Recharge, {"Recharge", 229}},
};

std::ostream &operator<<(std::ostream &os, Spell s)
{
	os << SPELL_MAP.at(s).first[0];
	return os;
}

// Wizards are complicated
int DoWizardTurn(Person &wizard, Spell attack, int *shield, int *poison, int *recharge)
{
	int w_attack = 0;
	if (wizard.man < SPELL_MAP.at(attack).second) {
		return -1;
	}
	wizard.man -= SPELL_MAP.at(attack).second;
	switch (attack) {
		case Spell::Missile:
			w_attack = 4;
			break;
		case Spell::Drain:
			w_attack = 2;
			wizard.hp += 2;
			break;
		case Spell::Shield:
			if (*shield > 0) return -1;
			*shield = 6;
			break;
		case Spell::Poison:
			if (*poison > 0) return -1;
			*poison = 6;
			break;
		case Spell::Recharge:
			if (*recharge > 0) return -1;
			*recharge = 5;
			break;
	}

	return w_attack;
}

void ProcessEffects(Person &w, Person &b, int * const poison, int * const shield, int * const recharge)
{
	if (*poison > 0) b.hp -= 3;
	if (*shield > 0) {
		w.arm = 7;
	} else {
		w.arm = 0; // reset after shield expired
	}
	if (*recharge > 0) w.man += 101;
	(*poison)--;
	(*shield)--;
	(*recharge)--;
}

int DoBattle(Person wizard, Person boss, const std::vector<Spell> &sequence, bool hardmode)
{
	int shield_timer = -1;
	int poison_timer = -1;
	int recharge_timer = -1;

	int mana_used = 0;

	for (auto attack : sequence) {
		// wizard turn
		ProcessEffects(wizard, boss, &poison_timer, &shield_timer, &recharge_timer);
		if (!boss.IsAlive()) return mana_used;
		if (hardmode) {
			wizard.hp--;
			if (!wizard.IsAlive()) return -1;
		}

		int w_att = DoWizardTurn(wizard, attack, &shield_timer, &poison_timer, &recharge_timer);
		if (w_att == -1) return -1; // invalid

		mana_used += SPELL_MAP.at(attack).second;
		boss.hp -= w_att;
		if (!boss.IsAlive()) return mana_used;

		// boss turn
		ProcessEffects(wizard, boss, &poison_timer, &shield_timer, &recharge_timer);
		if (!boss.IsAlive()) return mana_used;
		wizard.hp -= std::max(1, boss.dam - wizard.arm);
		if (!wizard.IsAlive()) return -1;
	}
	return -2; // not enough spells
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

	// Get boss stats
	std::string dummy;
	int b_hitpoints, b_damage;
	input >> dummy >> dummy >> b_hitpoints;
	input >> dummy >> b_damage;

	Person orig_boss(b_hitpoints, b_damage, 0, 0);
	Person orig_wizard(50, 0, 0, 500);

	int min_mana = 50000;
	int min_mana_hardmode = 50000;

	std::vector<std::vector<Spell>> sequences = {
		{Spell::Missile}, {Spell::Drain},
		{Spell::Shield}, {Spell::Poison},
		{Spell::Recharge}
	};
	while (!sequences.empty()) {
		if (sequences[0].size() > 10) break; // arbitrary
		std::vector<std::vector<Spell>> keep_sequences;
		for (const auto &sequence : sequences) {
			int mana_used = DoBattle(orig_wizard, orig_boss, sequence, false);
			int mana_used_hard = DoBattle(orig_wizard, orig_boss, sequence, true); // Part2
			if (mana_used == -2) {
				keep_sequences.push_back(sequence);
			} else if (mana_used > 0) {
				min_mana = std::min(min_mana, mana_used);
			}
			// Part2
			if (mana_used_hard > 0) min_mana_hardmode = std::min(min_mana_hardmode, mana_used_hard);
		}
		sequences.clear();
		for (auto seq : keep_sequences) {
			for (Spell s = Spell::First; s <= Spell::Last; s = (Spell)((int)s + 1)) {
				auto new_seq = seq;
				new_seq.push_back(s);
				sequences.push_back(new_seq);
			}
		}
	}
	std::cout << "Minimal mana needed to win: " << min_mana << "\n";
	std::cout << "Minimal mana needed to win (hardmode): " << min_mana_hardmode << "\n";
	return 0;
}
