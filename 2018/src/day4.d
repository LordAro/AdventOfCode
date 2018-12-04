import std.algorithm;
import std.file;
import std.format;
import std.stdio;

void main(string[] args)
{
	auto lines = slurp!(string)(args[1], "%s");
	sort(lines);

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
	int max_sleep_id = 0;
	int max_sleep = 0;
	int most_sleep_id;
	int most_minute;
	int most_minute_count;
	foreach (id, g; guards) {
		int sleep;
		int guard_most_min;
		int guard_most_count;
		foreach (m, c; g) {
			sleep += c;
			if (c > most_minute_count) {
				most_sleep_id = id;
				most_minute = m;
				most_minute_count = c;
			}
		}
		if (sleep > max_sleep) {
			max_sleep = sleep;
			max_sleep_id = id;
		}
	}
	int max_minute = 0;
	int max_minute_count = 0;
	foreach (m, c; guards[max_sleep_id]) {
		if (c > max_minute_count) {
			max_minute = m;
			max_minute_count = c;
		}
	}
	writeln("Strategy 1: ", max_minute * max_sleep_id);
	writeln("Strategy 2: ", most_minute * most_sleep_id);
}
