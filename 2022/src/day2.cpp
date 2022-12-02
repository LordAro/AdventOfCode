#include <fstream>
#include <iostream>

bool is_win(char opponent, char self)
{
	return (opponent == 0 && self == 1) || (opponent == 1 && self == 2) || (opponent == 2 && self == 0);
}

bool is_draw(char opponent, char self)
{
	return opponent == self;
}

char needed_move(char opponent, char outcome)
{
	switch (opponent) {
		case 0:
			switch (outcome) {
				case 0: return 2;
				case 1: return 0;
				case 2: return 1;
			}
			__builtin_unreachable();
		case 1:
			return outcome;
		case 2:
			switch (outcome) {
				case 0: return 1;
				case 1: return 2;
				case 2: return 0;
			}
			__builtin_unreachable();
	}
	__builtin_unreachable();
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

	// a = rock, b = paper, c = scissors
	// x = rock, y = paper, z = scissors

	// x = lose, y = draw, z = win

	int p1_score = 0;
	int p2_score = 0;

	char opponent_move_ch;
	char my_move_ch;
	while (input >> opponent_move_ch >> my_move_ch) {
		int opponent_move = opponent_move_ch - 'A';
		int my_move = my_move_ch - 'X';

		p1_score += my_move + 1;
		if (is_win(opponent_move, my_move)) {
			p1_score += 6;
		} else if (is_draw(opponent_move, my_move)) {
			p1_score += 3;
		}

		const int needed_outcome = my_move;
		int my_move2 = needed_move(opponent_move, needed_outcome);
		p2_score += my_move2 + 1;
		p2_score += needed_outcome * 3;
	}
	std::cout << "Total score (p1 definition): " << p1_score << '\n';
	std::cout << "Total score (p2 definition): " << p2_score << '\n';
}
