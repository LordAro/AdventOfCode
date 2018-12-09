import std.algorithm : maxElement;
import std.container : DList;
import std.file : slurp;
import std.stdio : writeln;

void rotate(ref DList!int dll, int count)
{
	if (count > 0) {
		for (int i = 0; i < count; i++) {
			dll.insertBack(dll.front);
			dll.removeFront();
		}
	} else {
		for (int i = 0; i < -count; i++) {
			dll.insertFront(dll.back);
			dll.removeBack();
		}
	}
}

ulong marble_game(int players, int last_marble)
{
	ulong[int] player_scores;
	DList!int marble_circle;
	marble_circle.insertBack(0);
	for (int marble = 1; marble <= last_marble; marble++) {
		if (marble % 23 == 0) {
			// Also remove previous increment
			rotate(marble_circle, 7);
			player_scores[marble % players] += marble_circle.front + marble;
			marble_circle.removeFront();
			rotate(marble_circle, -1);
		} else {
			rotate(marble_circle, -1);
			marble_circle.insertFront(marble);
		}
	}
	return player_scores.values.maxElement;
}

void main(string[] args)
{
	auto input = slurp!(int, int)(args[1], "%d players; last marble is worth %d points")[0];
	auto players = input[0];
	auto last_marble = input[1];

	writeln("High score: ", marble_game(players, last_marble));
	writeln("High score for bigger game: ", marble_game(players, last_marble * 100));
}

