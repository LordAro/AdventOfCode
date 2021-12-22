const std = @import("std");

const Coord = struct {
    x: usize,
    y: usize,
};

const PathCost = struct {
    coord: Coord,
    cost: u32,
};

fn lt(_: void, a: PathCost, b: PathCost) std.math.Order {
    return std.math.order(a.cost, b.cost);
}

fn find_path(alloc: std.mem.Allocator, grid: std.ArrayList(std.ArrayList(u8)), start: Coord, target: Coord) !u32 {
    var searched = std.AutoHashMap(Coord, void).init(alloc); // set
    defer searched.deinit();

    var toSearch = std.PriorityQueue(PathCost, void, lt).init(alloc, {});
    defer toSearch.deinit();
    try toSearch.add(PathCost{ .coord = start, .cost = 0 });
    var found_dist: u32 = @intCast(u32, grid.items.len) * @intCast(u32, grid.items.len);

    while (toSearch.items.len > 0) {
        const current = toSearch.remove();
        if (current.cost > found_dist) break; // no more possible smaller results, done

        if (searched.contains(current.coord)) continue;
        try searched.put(current.coord, {});

        if (current.coord.x == target.x and current.coord.y == target.y) {
            found_dist = current.cost;
        }

        // Adjacent coords. Could factor out into a loop, if I cared enough
        if (current.coord.x > 0) {
            const adj_coord = Coord{ .x = current.coord.x - 1, .y = current.coord.y };
            if (!searched.contains(adj_coord)) {
                const new_cost = current.cost + grid.items[adj_coord.y].items[adj_coord.x];
                try toSearch.add(PathCost{ .coord = adj_coord, .cost = new_cost });
            }
        }
        if (current.coord.x < grid.items[current.coord.y].items.len - 1) {
            const adj_coord = Coord{ .x = current.coord.x + 1, .y = current.coord.y };
            if (!searched.contains(adj_coord)) {
                const new_cost = current.cost + grid.items[adj_coord.y].items[adj_coord.x];
                try toSearch.add(PathCost{ .coord = adj_coord, .cost = new_cost });
            }
        }
        if (current.coord.y > 0) {
            const adj_coord = Coord{ .x = current.coord.x, .y = current.coord.y - 1 };
            if (!searched.contains(adj_coord)) {
                const new_cost = current.cost + grid.items[adj_coord.y].items[adj_coord.x];
                try toSearch.add(PathCost{ .coord = adj_coord, .cost = new_cost });
            }
        }
        if (current.coord.y < grid.items.len - 1) {
            const adj_coord = Coord{ .x = current.coord.x, .y = current.coord.y + 1 };
            if (!searched.contains(adj_coord)) {
                const new_cost = current.cost + grid.items[adj_coord.y].items[adj_coord.x];
                try toSearch.add(PathCost{ .coord = adj_coord, .cost = new_cost });
            }
        }
    }
    return found_dist;
}

pub fn main() anyerror!void {
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

    var grid = std.ArrayList(std.ArrayList(u8)).init(alloc);
    defer grid.deinit();

    var big_grid = std.ArrayList(std.ArrayList(u8)).init(alloc);
    defer big_grid.deinit();

    var buf: [128]u8 = undefined;
    while (try input.reader().readUntilDelimiterOrEof(&buf, '\n')) |line| {
        var row = std.ArrayList(u8).init(alloc);
        for (line) |c| {
            try row.append(try std.fmt.charToDigit(c, 10));
        }
        try grid.append(row);

        // Build big grid (extra cols)
        var long_row = std.ArrayList(u8).init(alloc);
        var i: u8 = 0;
        while (i < 5) : (i += 1) {
            for (row.items) |c| {
                try long_row.append((c + i) % 10 + (c + i) / 10); // wrap around, but nonzero
            }
        }
        try big_grid.append(long_row);
    }
    // Build big grid (extra rows)
    {
        var i: u8 = 1;
        while (i < 5) : (i += 1) {
            for (big_grid.items) |og_row, ix| {
                if (ix == 100) break;
                var long_row = std.ArrayList(u8).init(alloc);
                for (og_row.items) |c| {
                    try long_row.append((c + i) % 10 + (c + i) / 10); // wrap around, but nonzero
                }
                try big_grid.append(long_row);
            }
        }
    }

    const start = Coord{ .x = 0, .y = 0 };
    const p1_end = Coord{ .x = grid.items[0].items.len - 1, .y = grid.items.len - 1 };
    const p2_end = Coord{ .x = big_grid.items[0].items.len - 1, .y = big_grid.items.len - 1 };

    const path_cost = try find_path(alloc, grid, start, p1_end);
    const big_path_cost = try find_path(alloc, big_grid, start, p2_end);

    try stdout.print("Shortest path cost: {}\n", .{path_cost});
    try stdout.print("Shortest path cost for full grid: {}\n", .{big_path_cost});
}
