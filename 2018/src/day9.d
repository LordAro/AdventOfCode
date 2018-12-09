import std.algorithm : maxElement;
import std.container : DList;
import std.file : slurp;
import std.stdio : writeln;

void iterate_forwards(ref DList!int head, ref DList!int tail)
{
	if (tail.empty) {
		tail = head;
		head = DList!int();
	}
	head.insertBack(tail.front);
	tail.removeFront();
}

void iterate_backwards(ref DList!int head, ref DList!int tail)
{
	if (head.empty) {
		head = tail;
		tail = DList!int();
	}
	tail.insertFront(head.back);
	head.removeBack();
}

ulong marble_game(int players, int last_marble)
{
	ulong[int] player_scores;
	DList!int marble_circle_head, marble_circle_tail;
	marble_circle_head.insertBack(0);
	for (int marble = 1; marble <= last_marble; marble++) {
		auto cur_player = marble % players;

		if (marble % 23 == 0) {
			// Also remove previous increment
			for (int i = 0; i < 7 + 2; i++) {
				iterate_backwards(marble_circle_head, marble_circle_tail);
			}
			player_scores[cur_player] += marble;
			player_scores[cur_player] += marble_circle_tail.front;
			marble_circle_tail.removeFront();
		} else {
			marble_circle_tail.insertFront(marble);
		}

		for (int i = 0; i < 2; i++) {
			iterate_forwards(marble_circle_head, marble_circle_tail);
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

