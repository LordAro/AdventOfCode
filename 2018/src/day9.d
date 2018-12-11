import std.algorithm : maxElement;
import std.file : slurp;
import std.stdio : writeln;

struct Ring {
	struct Node {
		Node *prev;
		Node *next;
		int val;
	}

	private Node *current;
	Node[] memory;

	this(int init, size_t size)
	{
		this.current = new Node;
		this.memory = new Node[size]; // Use block of memory to save allocating little pieces
		this.current.prev = this.current;
		this.current.next = this.current;
		this.current.val = init;
	}

	int popVal()
	{
		auto n = this.current;
		n.prev.next = n.next;
		n.next.prev = n.prev;
		this.current = this.current.next;
		return n.val;
	}

	void insert(int val)
	{
		auto n = &this.memory[val];
		n.val = val;
		n.next = this.current;
		n.prev = this.current.prev;
		this.current.prev.next = n;
		this.current.prev = n;
		this.current = n;
	}

	void rotate(int count)
	{
		if (count > 0) {
			for (int i = 0; i < count; i++) {
				this.current = this.current.next;
			}
		} else {
			for (int i = 0; i < -count; i++) {
				this.current = this.current.prev;
			}
		}
	}
}

ulong marble_game(int players, int last_marble)
{
	ulong[int] player_scores;
	auto marble_circle = Ring(0, last_marble+1);
	for (int marble = 1; marble <= last_marble; marble++) {
		if (marble % 23 == 0) {
			marble_circle.rotate(7);
			player_scores[marble % players] += marble_circle.popVal() + marble;
			marble_circle.rotate(-1);
		} else {
			marble_circle.rotate(-1);
			marble_circle.insert(marble);
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

unittest
{
	Ring r = Ring(0);
	r.insert(1);
	assert(r.current.val == 1);
	assert(r.current.prev.val == 0);
	assert(r.current.next.val == 0);
}

unittest
{
	assert(marble_game(9, 25) == 32);
	assert(marble_game(17, 1104) == 2764);
	assert(marble_game(10, 1618) == 8317);
	assert(marble_game(30, 5807) == 37305);
	assert(marble_game(21, 6111) == 54718);
	assert(marble_game(13, 7999) == 146373);
}
