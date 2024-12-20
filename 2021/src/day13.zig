const std = @import("std");

const Coord = struct {
    x: u32,
    y: u32,
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

    var grid = std.AutoArrayHashMap(Coord, void).init(alloc); // set
    defer grid.deinit();

    var num_points: usize = 0;

    var parse_points: bool = true;
    var num_ins: u32 = 0;
    var buf: [300]u8 = undefined;
    while (try input.reader().readUntilDelimiterOrEof(&buf, '\n')) |line| {
        if (line.len == 0) {
            parse_points = false;
            continue;
        }

        if (parse_points) {
            var it = std.mem.split(u8, line, ",");
            const x = try std.fmt.parseInt(u32, it.next().?, 0);
            const y = try std.fmt.parseInt(u32, it.next().?, 0);
            try grid.put(Coord{ .x = x, .y = y }, {});
        } else {
            var it = std.mem.split(u8, line, " ");
            _ = it.next();
            _ = it.next(); // skip

            const ins = it.next().?;
            const dir = ins[0];
            const pos = try std.fmt.parseInt(u32, ins[2..], 0);

            var key_copy = std.ArrayList(Coord).init(alloc);
            try key_copy.appendSlice(grid.keys()[0..]);
            for (key_copy.items) |k| {
                if (dir == 'x' and k.x > pos) {
                    const new_k = Coord{ .x = pos - (k.x - pos), .y = k.y };
                    _ = grid.swapRemove(k);
                    try grid.put(new_k, {});
                } else if (dir == 'y' and k.y > pos) {
                    const new_k = Coord{ .x = k.x, .y = pos - (k.y - pos) };
                    _ = grid.swapRemove(k);
                    try grid.put(new_k, {});
                }
            }
            num_ins += 1;
        }

        if (num_ins == 1) {
            num_points = grid.keys().len;
        }
    }

    var max_x: u32 = 0;
    var max_y: u32 = 0;
    for (grid.keys()) |c| {
        max_x = @max(max_x, c.x);
        max_y = @max(max_y, c.y);
    }

    try stdout.print("Points after one fold: {}\n", .{num_points});
    try stdout.print("Code:\n", .{});
    var y: u32 = 0;
    while (y <= max_y) : (y += 1) {
        var x: u32 = 0;
        while (x <= max_x) : (x += 1) {
            const c: u8 = if (grid.contains(Coord{ .x = x, .y = y })) '#' else ' ';
            try stdout.print("{c}", .{c});
        }
        try stdout.print("\n", .{});
    }
}
