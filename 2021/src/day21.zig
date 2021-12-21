const std = @import("std");

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const alloc = &arena.allocator;
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

    var die_roll_n: u64 = 0; // 0-based

    var player1_score: u64 = 0;
    var player2_score: u64 = 0;

    while (true) {
        const die_roll1 = ((die_roll_n + 1) % 100) + ((die_roll_n + 2) % 100) + ((die_roll_n + 3) % 100);
        die_roll_n += 3;
        player1_pos = (player1_pos + die_roll1) % 10;
        player1_score += player1_pos + 1;
        if (player1_score >= 1000) break;

        const die_roll2 = ((die_roll_n + 1) % 100) + ((die_roll_n + 2) % 100) + ((die_roll_n + 3) % 100);
        die_roll_n += 3;
        player2_pos = (player2_pos + die_roll2) % 10;
        player2_score += player2_pos + 1;
        if (player2_score >= 1000) break;
    }

    std.debug.print("{} {} {}\n", .{ player1_score, player2_score, die_roll_n });

    try stdout.print("Losing player score: {}, number of dice rolls: {} ({})\n", .{ std.math.min(player1_score, player2_score), die_roll_n, std.math.min(player1_score, player2_score) * die_roll_n });
}
