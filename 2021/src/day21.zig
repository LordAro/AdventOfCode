const std = @import("std");

const P1Result = struct {
    p1: u64,
    p2: u64,
    die_rolls: u64,
};

fn p1(p1_start: u64, p2_start: u64) P1Result {
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

const WinCounter = struct {
    p1: u64,
    p2: u64,
};

// N-1
fn roll_n(n: u64) u64 {
    return switch (n) {
        2, 8 => 1,
        3, 7 => 3,
        4, 6 => 6,
        5 => 7,
        else => unreachable,
    };
}

fn p2(p1_start: u64, p2_start: u64, p1_score: u64, p2_score: u64) WinCounter {
    // 3
    // 444
    // 555555
    // 6666666
    // 777777
    // 888
    // 9

    if (p1_score >= 21) {
        return .{ .p1 = 1, .p2 = 0 };
    } else if (p2_score >= 21) {
        return .{ .p1 = 0, .p2 = 1 };
    }

    var p1_wins: u64 = 0;
    var p2_wins: u64 = 0;
    var roll_1: u64 = 2; // Note: n-1 for 0-indexing
    while (roll_1 <= 8) : (roll_1 += 1) {
        const multiplier1 = roll_n(roll_1);
        const new_p1_pos = (p1_start + roll_1) % 10;
        var roll_2: u64 = 2;
        while (roll_2 <= 8) : (roll_2 += 1) {
            const multiplier2 = roll_n(roll_2);
            const new_p2_pos = (p2_start + roll_2) % 10;
            const counter = p2(new_p1_pos, new_p2_pos, p1_score + new_p1_pos + 1, p2_score + new_p2_pos + 1);
            p1_wins += multiplier1 * multiplier2 * counter.p1; // ???
            p2_wins += multiplier1 * multiplier2 * counter.p2;
        }
    }

    const res: WinCounter = .{ .p1 = p1_wins, .p2 = p2_wins };
    std.debug.print("{any}\n", .{res});
    return res;
}

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const alloc = arena.allocator();
    const stdout = std.io.getStdOut().writer();

    var args_iter = std.process.args();
    _ = args_iter.skip(); // program name
    const input_file = try args_iter.next(alloc) orelse unreachable;
    defer alloc.free(input_file);
    const input = std.fs.cwd().openFile(input_file, .{ .read = true }) catch |err| {
        std.log.err("Could not open {s} due to: {s}", .{ input_file, err });
        std.os.exit(1);
    };
    defer input.close();

    var player1_pos: u64 = 0;
    var player2_pos: u64 = 0;

    var input_reader = input.reader();
    var buf: [64]u8 = undefined;
    if (try input_reader.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        const space = std.mem.lastIndexOfScalar(u8, line, ' ').?;
        player1_pos = (try std.fmt.parseInt(u8, line[space + 1 ..], 0)) - 1; // 0-based position
    }
    if (try input_reader.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        const space = std.mem.lastIndexOfScalar(u8, line, ' ').?;
        player2_pos = (try std.fmt.parseInt(u8, line[space + 1 ..], 0)) - 1;
    }

    player1_pos = 3;
    player2_pos = 7;

    const p1result = p1(player1_pos, player2_pos);

    const p2result = p2(player1_pos, player2_pos, 0, 0);

    try stdout.print("Losing player score: {}, number of dice rolls: {} ({})\n", .{ std.math.min(p1result.p1, p1result.p2), p1result.die_rolls, std.math.min(p1result.p1, p1result.p2) * p1result.die_rolls });
    try stdout.print("P1 winning: {}, P2 winning: {}\n", .{ p2result.p1, p2result.p2 });
}
