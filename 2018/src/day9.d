import std.algorithm : remove, maxElement;
import std.array : insertInPlace;
import std.file : slurp;
import std.stdio : writeln;

int marble_game(int players, int last_marble)
{
	int[int] player_scores;
	int[] marble_circle = [0];
	marble_circle.reserve(last_marble + 1);
	size_t marb_idx = 0;
	for (int marble = 1; marble <= last_marble; marble++) {
		auto cur_player = marble % players;

		if (marble % 23 == 0) {
			// Also remove previous increment
			marb_idx = (marb_idx - 7 - 2 + marble_circle.length) % marble_circle.length;
			player_scores[cur_player] += marble;
			player_scores[cur_player] += marble_circle[marb_idx + 1];
			marble_circle = marble_circle.remove(marb_idx + 1);
		} else {
			marble_circle.insertInPlace(marb_idx + 1, marble);
		}

	//	writeln("[", cur_player, "] Cur: ", marb_idx, " ", marble_circle);

		marb_idx = (marb_idx + 2) % marble_circle.length;
	}
	return player_scores.values.maxElement;
}

void main(string[] args)
{
	auto input = slurp!(int, int)(args[1], "%d players; last marble is worth %d points")[0];
	auto players = input[0];
	auto last_marble = input[1];

	writeln("High score: ", marble_game(players, last_marble));
	//writeln("High score for bigger game: ", marble_game(players, last_marble * 100));
}

