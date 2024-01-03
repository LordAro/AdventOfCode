const std = @import("std");

fn illegal_char_score(c: u8) u32 {
    return switch (c) {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        else => unreachable,
    };
}

fn closed(c: u8) u8 {
    return switch (c) {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        else => unreachable,
    };
}

fn autocomplete_char_score(c: u8) u32 {
    return switch (c) {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        else => unreachable,
    };
}

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
        std.os.exit(1);
    };
    defer input.close();

    var illegal_score: u32 = 0;

    var autocomplete_scores = std.ArrayList(u64).init(alloc);
    defer autocomplete_scores.deinit();

    var buf: [128]u8 = undefined;
    while (try input.reader().readUntilDelimiterOrEof(&buf, '\n')) |line| {
        var stack = std.ArrayList(u8).init(alloc);
        defer stack.deinit();
        var is_illegal = false;
        for (line) |c| {
            switch (c) {
                '(', '[', '{', '<' => try stack.append(c),
                ')', ']', '}', '>' => {
                    const tail = stack.pop();
                    if (closed(tail) != c) {
                        illegal_score += illegal_char_score(c);
                        is_illegal = true;
                        break;
                    }
                },
                else => unreachable,
            }
        }

        if (!is_illegal) {
            var autocomplete_score: u64 = 0;
            while (stack.items.len != 0) {
                const tail = stack.pop();
                autocomplete_score = autocomplete_score * 5 + autocomplete_char_score(tail);
            }
            try autocomplete_scores.append(autocomplete_score);
        }
    }

    std.mem.sort(u64, autocomplete_scores.items, {}, comptime std.sort.asc(u64));

    const middle_score = autocomplete_scores.items[(autocomplete_scores.items.len - 1) / 2];

    try stdout.print("Illegal character score: {}\n", .{illegal_score});
    try stdout.print("Median autocomplete score: {}\n", .{middle_score});
}
