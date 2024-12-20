const std = @import("std");

fn flood_fill(grid: anytype, contents: *std.AutoHashMap(u32, void), x: usize, y: usize) void {
    const cell = grid.items[y].items[x];
    if (contents.*.contains(@as(u32, @intCast(y * grid.items.len + x)))) {
        return;
    }
    if (cell == 9) {
        return;
    }
    contents.*.put(@as(u32, @intCast(y * grid.items.len + x)), {}) catch unreachable;
    if (x > 0) {
        flood_fill(grid, contents, x - 1, y);
    }
    if (x < grid.items[y].items.len - 1) {
        flood_fill(grid, contents, x + 1, y);
    }
    if (y > 0) {
        flood_fill(grid, contents, x, y - 1);
    }
    if (y < grid.items.len - 1) {
        flood_fill(grid, contents, x, y + 1);
    }
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
        std.process.exit(1);
    };
    defer input.close();

    var grid = std.ArrayList(std.ArrayList(u8)).init(alloc);
    defer grid.deinit();

    var buf: [128]u8 = undefined;
    while (try input.reader().readUntilDelimiterOrEof(&buf, '\n')) |line| {
        var row = std.ArrayList(u8).init(alloc);
        for (line) |c| {
            try row.append(try std.fmt.charToDigit(c, 10));
        }
        try grid.append(row);
    }

    var risk_level: u32 = 0;
    var basin_sizes = std.ArrayList(u32).init(alloc);

    for (grid.items, 0..) |row, y| {
        for (row.items, 0..) |cell, x| {
            const upper = if (y == 0) 10 else grid.items[y - 1].items[x];
            const lower = if (y == grid.items.len - 1) 10 else grid.items[y + 1].items[x];
            const left = if (x == 0) 10 else grid.items[y].items[x - 1];
            const right = if (x == row.items.len - 1) 10 else grid.items[y].items[x + 1];

            if (cell < upper and cell < lower and cell < left and cell < right) {
                risk_level += 1 + cell; // 1 + height
                var basin = std.AutoHashMap(u32, void).init(alloc);
                defer basin.deinit();
                flood_fill(grid, &basin, x, y);
                //std.debug.print("Flood fill result: {},{} -> {any}\n", .{ x, y, basin });
                try basin_sizes.append(basin.count());
            }
        }
    }

    std.mem.sort(u32, basin_sizes.items, {}, comptime std.sort.desc(u32));

    try stdout.print("Total risk level of low points: {}\n", .{risk_level});
    try stdout.print("Three largest basins: {} {} {} ({})\n", .{ basin_sizes.items[0], basin_sizes.items[1], basin_sizes.items[2], basin_sizes.items[0] * basin_sizes.items[1] * basin_sizes.items[2] });
}
