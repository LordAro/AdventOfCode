const std = @import("std");

const Coord = struct {
    x: u32,
    y: u32,
};

fn parse_coord(str: []const u8) Coord {
    var it = std.mem.split(u8, str, ",");
    const x = std.fmt.parseInt(u32, it.next().?, 0) catch unreachable;
    const y = std.fmt.parseInt(u32, it.next().?, 0) catch unreachable;
    return Coord{ .x = x, .y = y };
}

fn add_point(map: anytype, c: Coord) !void {
    const gop = try map.getOrPut(c);
    if (!gop.found_existing) {
        gop.value_ptr.* = 0;
    }
    gop.value_ptr.* += 1;
}

const LineIterator = struct {
    start: Coord,
    end: Coord,
    step_x: i32,
    step_y: i32,
    next_val: Coord,

    pub fn init(start: Coord, end: Coord) LineIterator {
        var step_x: i32 = 0;
        if (start.x == end.x) {
            step_x = 0;
        } else if (start.x > end.x) {
            step_x = -1;
        } else {
            step_x = 1;
        }
        var step_y: i32 = 0;
        if (start.y == end.y) {
            step_y = 0;
        } else if (start.y > end.y) {
            step_y = -1;
        } else {
            step_y = 1;
        }
        return LineIterator{
            .next_val = start,
            .start = start,
            .end = end,
            .step_x = step_x,
            .step_y = step_y,
        };
    }

    pub fn next(self: *LineIterator) ?Coord {
        const rv = self.next_val;
        if ((self.step_x < 0 and rv.x < self.end.x) or (self.step_y < 0 and rv.y < self.end.y)) {
            // Done counting down
            return null;
        } else if ((self.step_x > 0 and rv.x > self.end.x) or (self.step_y > 0 and rv.y > self.end.y)) {
            // Done counting up
            return null;
        }
        self.next_val = Coord{ .x = @intCast(u32, @intCast(i32, rv.x) + self.step_x), .y = @intCast(u32, @intCast(i32, rv.y) + self.step_y) };
        return rv;
    }

    pub fn reset(self: *LineIterator) void {
        self.next_val = self.start;
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
        std.os.exit(1);
    };
    defer input.close();

    var map = std.AutoArrayHashMap(Coord, u32).init(alloc);
    defer map.deinit();
    var diag_map = std.AutoArrayHashMap(Coord, u32).init(alloc);
    defer diag_map.deinit();

    var buf: [64]u8 = undefined;
    while (try input.reader().readUntilDelimiterOrEof(&buf, '\n')) |line| {
        var it = std.mem.split(u8, line, " ");
        var coord1 = parse_coord(it.next().?);
        _ = it.next();
        var coord2 = parse_coord(it.next().?);

        const is_diagonal = coord1.x != coord2.x and coord1.y != coord2.y;
        var line_it = LineIterator.init(coord1, coord2);
        while (line_it.next()) |c| {
            if (is_diagonal) {
                try add_point(&diag_map, c);
            } else {
                try add_point(&diag_map, c);
                try add_point(&map, c);
            }
        }
    }

    var count: u32 = 0;
    for (map.values()) |val| {
        if (val > 1) {
            count += 1;
        }
    }

    var count2: u32 = 0;
    for (diag_map.values()) |val| {
        if (val > 1) {
            count2 += 1;
        }
    }
    try stdout.print("Duplicate vent count (straight lines): {}\n", .{count});
    try stdout.print("Duplicate vent count (diagonals): {}\n", .{count2});
}

test "line coords" {
    var it = LineIterator.init(Coord{ .x = 1, .y = 1 }, Coord{ .x = 1, .y = 3 });
    try std.testing.expectEqual(it.next().?, Coord{ .x = 1, .y = 1 });
    try std.testing.expectEqual(it.next().?, Coord{ .x = 1, .y = 2 });
    try std.testing.expectEqual(it.next().?, Coord{ .x = 1, .y = 3 });
    try std.testing.expectEqual(it.next(), null);
}

test "line coords 2" {
    var it = LineIterator.init(Coord{ .x = 9, .y = 7 }, Coord{ .x = 7, .y = 7 });
    try std.testing.expectEqual(it.next().?, Coord{ .x = 9, .y = 7 });
    try std.testing.expectEqual(it.next().?, Coord{ .x = 8, .y = 7 });
    try std.testing.expectEqual(it.next().?, Coord{ .x = 7, .y = 7 });
    try std.testing.expectEqual(it.next(), null);
}

test "line coords 3" {
    var it = LineIterator.init(Coord{ .x = 9, .y = 7 }, Coord{ .x = 7, .y = 9 });
    try std.testing.expectEqual(it.next().?, Coord{ .x = 9, .y = 7 });
    try std.testing.expectEqual(it.next().?, Coord{ .x = 8, .y = 8 });
    try std.testing.expectEqual(it.next().?, Coord{ .x = 7, .y = 9 });
    try std.testing.expectEqual(it.next(), null);
}
