const std = @import("std");

const Coord = struct {
    x: u32,
    y: u32,
};

fn parse_coord(str: anytype) Coord {
    var it = std.mem.split(str, ",");
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

pub fn main() anyerror!void {
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

    var map = std.AutoHashMap(Coord, u32).init(alloc);
    defer map.deinit();
    var diag_map = std.AutoHashMap(Coord, u32).init(alloc);
    defer diag_map.deinit();

    var buf: [64]u8 = undefined;
    while (try input.reader().readUntilDelimiterOrEof(&buf, '\n')) |line| {
        var it = std.mem.split(line, " "); // tokenize squashes duplicate delimiters
        var coord1 = parse_coord(it.next().?);
        _ = it.next();
        var coord2 = parse_coord(it.next().?);

        const x_inc: i32 = if (coord1.x > coord2.x) -1 else 1;
        const y_inc: i32 = if (coord1.y > coord2.y) -1 else 1;
        if (coord1.x != coord2.x and coord1.y != coord2.y) {
            var x = coord1.x;
            var y = coord1.y;
            while (x != coord2.x) {
                const c = Coord{ .x = x, .y = y };
                try add_point(&diag_map, c);
                x = @intCast(u32, @intCast(i32, x) + x_inc);
                y = @intCast(u32, @intCast(i32, y) + y_inc);
            }
            try add_point(&diag_map, coord2);
        } else if (coord1.x != coord2.x) {
            var x = coord1.x;
            while (x != coord2.x) : (x = @intCast(u32, @intCast(i32, x) + x_inc)) {
                const c = Coord{ .x = x, .y = coord1.y };
                try add_point(&map, c);
                try add_point(&diag_map, c);
            }
            try add_point(&map, coord2);
            try add_point(&diag_map, coord2);
        } else if (coord1.y != coord2.y) {
            var y = coord1.y;
            while (y != coord2.y) : (y = @intCast(u32, @intCast(i32, y) + y_inc)) {
                const c = Coord{ .x = coord1.x, .y = y };
                try add_point(&map, c);
                try add_point(&diag_map, c);
            }
            try add_point(&map, coord2);
            try add_point(&diag_map, coord2);
        }
    }

    var count: u32 = 0;
    var it = map.valueIterator();
    while (it.next()) |val| {
        if (val.* > 1) {
            count += 1;
        }
    }

    var count2: u32 = 0;
    var it2 = diag_map.valueIterator();
    while (it2.next()) |val| {
        if (val.* > 1) {
            count2 += 1;
        }
    }
    try stdout.print("Duplicate vent count (straight lines): {}\n", .{count});
    try stdout.print("Duplicate vent count (diagonals): {}\n", .{count2});
}
