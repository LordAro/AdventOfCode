const std = @import("std");

pub fn main() anyerror!void {
    const stdout = std.io.getStdOut().writer();

    var args_iter = std.process.args();
    _ = args_iter.skip(); // program name
    const input_file = args_iter.next() orelse unreachable;
    const input = std.fs.cwd().openFile(input_file, .{}) catch |err| {
        std.log.err("Could not open {s} due to: {}", .{ input_file, err });
        std.process.exit(1);
    };
    defer input.close();

    const lanternSpawnRate = 7;
    const lanternInitialSpawn = 9;
    const dayLimit = 256;

    var spawnDays: [dayLimit]u64 = [_]u64{0} ** dayLimit;

    var initial_input_count: u32 = 0;
    var buf: [4]u8 = undefined;
    while (try input.reader().readUntilDelimiterOrEof(&buf, ',')) |chr| {
        initial_input_count += 1;
        // Use chr[0] to make sure a trailing \n does not cause errors (and charToDigit instead of parseInt is better anyway)
        const n = try std.fmt.charToDigit(chr[0], 10);

        var i: usize = 0;
        while (n + i * lanternSpawnRate < dayLimit) : (i += 1) {
            spawnDays[n + i * lanternSpawnRate] += 1;
        }
    }

    var day: u32 = 1;
    while (day < dayLimit) : (day += 1) {
        var i: usize = 0;
        while (day + lanternInitialSpawn + i * lanternSpawnRate < dayLimit) : (i += 1) {
            spawnDays[day + lanternInitialSpawn + i * lanternSpawnRate] += spawnDays[day];
        }
    }

    var sum80: u64 = initial_input_count;
    for (spawnDays[0..80]) |n| {
        sum80 += n;
    }
    var sum256: u64 = initial_input_count;
    for (spawnDays[0..256]) |n| {
        sum256 += n;
    }
    try stdout.print("Number of lanternfish after 80 days: {}\n", .{sum80});
    try stdout.print("After 256 days: {}\n", .{sum256});
}
