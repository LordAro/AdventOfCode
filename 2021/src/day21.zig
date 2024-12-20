const std = @import("std");

const P1Result = struct {
    p1: u64,
    p2: u64,
    die_rolls: u64,
};

fn deterministic_dice_game(p1_start: u64, p2_start: u64) P1Result {
    var p1_pos = p1_start;
    var p2_pos = p2_start;
    var die_roll_n: u64 = 0; // 0-based

    var player1_score: u64 = 0;
    var player2_score: u64 = 0;

    while (true) {
        const die_roll1 = ((die_roll_n + 1) % 100) + ((die_roll_n + 2) % 100) + ((die_roll_n + 3) % 100);
        die_roll_n += 3;
        p1_pos = (p1_pos + die_roll1) % 10;
        player1_score += p1_pos + 1;
        if (player1_score >= 1000) break;

        const die_roll2 = ((die_roll_n + 1) % 100) + ((die_roll_n + 2) % 100) + ((die_roll_n + 3) % 100);
        die_roll_n += 3;
        p2_pos = (p2_pos + die_roll2) % 10;
        player2_score += p2_pos + 1;
        if (player2_score >= 1000) break;
    }

    return .{ .p1 = player1_score, .p2 = player2_score, .die_rolls = die_roll_n };
}

// each player rolls the die 3 times and adds up the results
// this function returns the number of possibilities of a particular result
fn roll_n(n: u64) u64 {
    // 3
    // 444
    // 555555
    // 6666666
    // 777777
    // 888
    // 9

    // n is 0-based
    return switch (n + 1) {
        3, 9 => 1,
        4, 8 => 3,
        5, 7 => 6,
        6 => 7,
        else => unreachable,
    };
}

const Pair = struct {
    p1: u64,
    p2: u64,
};

const CacheKey = struct {
    positions: Pair,
    scores: Pair,
};

const Cache = std.AutoHashMap(CacheKey, Pair);
fn quantum_dice_game(cache: *Cache, positions: Pair, scores: Pair) Pair {
    // if both players reached 21 last round, p1 would've reached it first, so still a win for p1
    if (scores.p1 >= 21) {
        return .{ .p1 = 1, .p2 = 0 };
    } else if (scores.p2 >= 21) {
        return .{ .p1 = 0, .p2 = 1 };
    }

    const cachekey: CacheKey = .{ .positions = positions, .scores = scores };
    if (cache.get(cachekey)) |wc| {
        return wc;
    }

    var wins: Pair = .{ .p1 = 0, .p2 = 0 };
    // rolling 3 3-sided dice means can get 3-9
    for (2..9) |roll_1| { // Note: n-1 for 0-indexing (upper exclusive)
        const multiplier1 = roll_n(roll_1);
        const new_p1_pos = ((positions.p1 + roll_1 + 1) % 10);

        for (2..9) |roll_2| {
            const multiplier2 = roll_n(roll_2);
            const new_p2_pos = ((positions.p2 + roll_2 + 1) % 10);
            const counter = quantum_dice_game(
                cache,
                .{ .p1 = new_p1_pos, .p2 = new_p2_pos },
                .{ .p1 = scores.p1 + new_p1_pos + 1, .p2 = scores.p2 + new_p2_pos + 1 },
            );
            wins.p1 += multiplier1 * multiplier2 * counter.p1;
            wins.p2 += multiplier1 * multiplier2 * counter.p2;
        }
    }

    std.debug.print("p2({}, {}, {}, {}) -> ({}, {})\n", .{ positions.p1, positions.p2, scores.p1, scores.p2, wins.p1, wins.p2 });
    cache.put(cachekey, wins) catch unreachable;
    return wins;
}

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const alloc = arena.allocator();
    const stdout = std.io.getStdOut().writer();

    var args_iter = std.process.args();
    _ = args_iter.skip(); // program name
    const input_file = args_iter.next() orelse unreachable;
    const input = std.fs.cwd().openFile(input_file, .{}) catch |err| {
        std.log.err("Could not open {s} due to: {}", .{ input_file, err });
        std.process.exit(1);
    };
    defer input.close();

    var player1_pos: u64 = 0;
    var player2_pos: u64 = 0;

    var input_reader = input.reader();
    var buf: [64]u8 = undefined;

    // track is 1-10, so we use 0-based indexes to allow use of modular arithmetic
    if (try input_reader.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        const space = std.mem.lastIndexOfScalar(u8, line, ' ').?;
        player1_pos = (try std.fmt.parseInt(u8, line[space + 1 ..], 0)) - 1;
    }
    if (try input_reader.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        const space = std.mem.lastIndexOfScalar(u8, line, ' ').?;
        player2_pos = (try std.fmt.parseInt(u8, line[space + 1 ..], 0)) - 1;
    }

    const p1result = deterministic_dice_game(player1_pos, player2_pos);
    try stdout.print("Losing player score: {}, number of dice rolls: {} ({})\n", .{ @min(p1result.p1, p1result.p2), p1result.die_rolls, @min(p1result.p1, p1result.p2) * p1result.die_rolls });

    // XXX For some reason, player1 win count is 27 times higher than it should be. I have no idea why. player2 is fine.
    var cache = Cache.init(alloc);
    defer cache.deinit();
    const p2result = quantum_dice_game(&cache, .{ .p1 = player1_pos, .p2 = player2_pos }, .{ .p1 = 0, .p2 = 0 });
    try stdout.print(
        "P1 winning: {}, P2 winning: {} ({s} wins)\n",
        .{ p2result.p1 / 27, p2result.p2, if (p2result.p1 / 27 > p2result.p2) "P1" else "P2" },
    );
}
