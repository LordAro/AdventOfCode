#include <algorithm>
#include <fstream>
#include <iostream>
#include <vector>

bool is_win(char opponent, char self)
{
	return (opponent == 'A' && self == 'Y') || (opponent == 'B' && self == 'Z') || (opponent == 'C' && self == 'X');
}

bool is_draw(char opponent, char self)
{
	return (opponent == 'A' && self == 'X') || (opponent == 'B' && self == 'Y') || (opponent == 'C' && self == 'Z');
}

char needed_move(char opponent, char outcome)
{
	switch (opponent) {
		case 'A':
			switch (outcome) {
				case 'X': return 'Z';
				case 'Y': return 'X';
				case 'Z': return 'Y';
			}
			__builtin_unreachable();
		case 'B':
			switch (outcome) {
				case 'X': return 'X';
				case 'Y': return 'Y';
				case 'Z': return 'Z';
			}
			__builtin_unreachable();
		case 'C':
			switch (outcome) {
				case 'X': return 'Y';
				case 'Y': return 'Z';
				case 'Z': return 'X';
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

	char opponent_move;
	char my_move;
	while (input >> opponent_move, input >> my_move) { // mm, commas
		p1_score += (my_move - 'X' + 1);
		if (is_win(opponent_move, my_move)) {
			p1_score += 6;
		} else if (is_draw(opponent_move, my_move)) {
			p1_score += 3;
		}

		const char needed_outcome = my_move;
		char my_move2 = needed_move(opponent_move, needed_outcome);
		p2_score += (my_move2 - 'X' + 1);
		p2_score += (needed_outcome - 'X') * 3;
	}
	std::cout << "Total score (p1 definition): " << p1_score << '\n';
	std::cout << "Total score (p2 definition): " << p2_score << '\n';
}
