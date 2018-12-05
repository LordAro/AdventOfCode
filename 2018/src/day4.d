import std.algorithm;
import std.array;
import std.file;
import std.format;
import std.stdio;
import std.typecons;

void main(string[] args)
{
	auto lines = slurp!(string)(args[1], "%s");
	sort(lines);

	// Id -> asleep minute -> count
	int[int][int] guards;
	int current_id = 0;
	int sleep_time = 0;
	foreach (line; lines) {
		auto splitted = line.split;
		if (splitted.length == 6) {
			formattedRead(splitted[3], "#%d", &current_id);
		} else if (splitted[2] == "falls") {
			formattedRead(splitted[1], "00:%d]", &sleep_time);
		} else {
			int wake_time;
			formattedRead(splitted[1], "00:%d]", &wake_time);
			for (int i = sleep_time; i < wake_time; i++) {
				guards[current_id][i]++;
			}
		}
	}

	// id[minute][count] -> tuple(id, minute[count]) -> tuple(id, sum(minute[count])) -> (max(sum)) -> id
	auto sleepiest_guard = guards.byPair.map!(a => tuple(a[0], sum(a[1].byValue))).maxElement!(a => a[1])[0];
	// minute[count] -> tuple(minute, count) -> (max(count)) -> minute
	auto sleepiest_minute = guards[sleepiest_guard].byPair.maxElement!(a => a[1])[0];

	// id[minute][count] -> tuple(id, tuple(minute, max(count)))
	auto most_sleepy = guards.byPair.map!(g => tuple(g[0], g[1].byPair.maxElement!(a => a[1]))).maxElement!(g => g[1][1]);
	auto most_sleepy_minute = most_sleepy[1][0];
	auto most_sleepy_guard = most_sleepy[0];

	writeln("Strategy 1: ", sleepiest_guard, "*", sleepiest_minute, "=", sleepiest_guard * sleepiest_minute);
	writeln("Strategy 2: ", most_sleepy_guard, "*", most_sleepy_minute, "=", most_sleepy_guard * most_sleepy_minute);
}
