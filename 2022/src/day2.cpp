#include <fstream>
#include <iostream>

int result(int opponent, int self)
{
	// correctly returns 0 = lose, 1 = draw, 2 = win... somehow
	return (self - opponent + 1 + 3) % 3;
}

int needed_move(int opponent, int outcome)
{
	// returns the needed move to get desired outcome
	return (opponent + outcome + 2) % 3;
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
		p1_score += result(opponent_move, my_move) * 3;

		const int needed_outcome = my_move;
		int my_move2 = needed_move(opponent_move, needed_outcome);
		p2_score += my_move2 + 1;
		p2_score += needed_outcome * 3;
	}
	std::cout << "Total score (p1 definition): " << p1_score << '\n';
	std::cout << "Total score (p2 definition): " << p2_score << '\n';
}
