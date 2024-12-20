const std = @import("std");

const Board = struct {
    b: [5][5]u32 = [_][5]u32{[_]u32{0} ** 5} ** 5,
    p: [5][5]bool = [_][5]bool{[_]bool{false} ** 5} ** 5,

    fn is_completed(self: *const Board) bool {
        var i: usize = 0;
        while (i < 5) : (i += 1) {
            if (self.p[i][0] and self.p[i][1] and self.p[i][2] and self.p[i][3] and self.p[i][4]) {
                return true;
            }
            if (self.p[0][i] and self.p[1][i] and self.p[2][i] and self.p[3][i] and self.p[4][i]) {
                return true;
            }
        }
        return false;
    }

    fn pick_number(self: *Board, n: u32) bool {
        var i: usize = 0;
        while (i < 25) : (i += 1) {
            if (self.b[i / 5][i % 5] == n) {
                self.p[i / 5][i % 5] = true;
                return true;
            }
        }
        return false;
    }

    fn sum_unpicked(self: *const Board) u32 {
        var sum: u32 = 0;
        var i: usize = 0;
        while (i < 25) : (i += 1) {
            if (!self.p[i / 5][i % 5]) {
                sum += self.b[i / 5][i % 5];
            }
        }
        return sum;
    }
};

pub fn main() anyerror!void {
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

    var picked_numbers = std.ArrayList(u32).init(alloc);
    defer picked_numbers.deinit();
    var boards = std.ArrayList(Board).init(alloc);
    defer boards.deinit();

    var first_line: bool = true;
    var board_line: u32 = 0;
    var buf: [300]u8 = undefined;
    while (try input.reader().readUntilDelimiterOrEof(&buf, '\n')) |line| {
        if (first_line) {
            var it = std.mem.split(u8, line, ",");
            while (it.next()) |n_str| {
                try picked_numbers.append(try std.fmt.parseInt(u32, n_str, 0));
            }
            first_line = false;
            continue;
        }

        if (line.len == 0) {
            board_line = 0;
        } else {
            if (board_line == 0) {
                try boards.append(Board{});
            }
            const b = &boards.items[boards.items.len - 1];

            var it = std.mem.tokenize(u8, line, " "); // tokenize squashes duplicate delimiters
            var i: usize = 0;
            while (it.next()) |n_str| : (i += 1) {
                b.b[board_line][i] = try std.fmt.parseInt(u32, n_str, 0);
            }

            board_line += 1;
        }
    }

    const Score = struct {
        last_num: u32 = 0,
        unpicked_sum: u32 = 0,
    };

    var first_win: Score = .{};
    var last_win: Score = .{};
    var win_count: usize = 0;
    outer: for (picked_numbers.items) |num| {
        for (boards.items) |*board| {
            if (!board.is_completed() and board.*.pick_number(num)) {
                if (board.is_completed()) {
                    win_count += 1;
                    last_win = .{ .last_num = num, .unpicked_sum = board.sum_unpicked() };
                    if (win_count == 1) {
                        first_win = last_win;
                    } else if (win_count == boards.items.len) {
                        break :outer;
                    }
                }
            }
        }
    }

    try stdout.print("First win score: {} {} ({})\n", .{ first_win.last_num, first_win.unpicked_sum, first_win.last_num * first_win.unpicked_sum });
    try stdout.print("Last win score: {} {} ({})\n", .{ last_win.last_num, last_win.unpicked_sum, last_win.last_num * last_win.unpicked_sum });
}
