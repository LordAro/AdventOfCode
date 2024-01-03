const std = @import("std");

fn dup(alloc: std.mem.Allocator, s: []const u8) []u8 {
    var newStr = alloc.alloc(u8, s.len) catch unreachable;
    std.mem.copy(u8, newStr, s);
    return newStr;
}

fn is_lowercase(s: []const u8) bool {
    for (s) |c| {
        if (!std.ascii.isLower(c)) return false;
    }
    return true;
}

fn arr_contains(arr: std.ArrayList([]const u8), s: []const u8) bool {
    for (arr.items) |item| {
        if (std.mem.eql(u8, item, s)) return true;
    }
    return false;
}

fn has_dup_lowercase(alloc: std.mem.Allocator, arr: std.ArrayList([]const u8)) bool {
    var uniq = std.StringHashMap(void).init(alloc); // set
    defer uniq.deinit();
    for (arr.items) |r| {
        if (!is_lowercase(r)) continue;
        var gop = uniq.getOrPut(r) catch unreachable;
        if (gop.found_existing) {
            return true;
        }
    }
    return false;
}

const Graph = std.StringArrayHashMap(std.ArrayList([]u8));

fn bfs(G: *const Graph, alloc: std.mem.Allocator, route: std.ArrayList([]const u8)) u32 {
    const tail = route.items[route.items.len - 1];
    if (std.mem.eql(u8, tail, "end")) return 1;

    var num_routes: u32 = 0;
    for (G.get(tail).?.items) |child| {
        if (is_lowercase(child) and arr_contains(route, child)) continue;
        var new_route = std.ArrayList([]const u8).init(alloc);
        defer new_route.deinit();
        new_route.appendSlice(route.items[0..]) catch unreachable;
        new_route.append(child) catch unreachable;
        num_routes += bfs(G, alloc, new_route);
    }

    return num_routes;
}

fn bfs2(G: *const Graph, alloc: std.mem.Allocator, route: std.ArrayList([]const u8)) u32 {
    const tail = route.items[route.items.len - 1];
    if (std.mem.eql(u8, tail, "end")) {
        //std.debug.print("{s}\n", .{route.items});
        return 1;
    }

    var num_routes: u32 = 0;
    for (G.get(tail).?.items) |child| {
        if (is_lowercase(child) and has_dup_lowercase(alloc, route) and arr_contains(route, child)) continue;
        var new_route = std.ArrayList([]const u8).init(alloc);
        defer new_route.deinit();
        new_route.appendSlice(route.items[0..]) catch unreachable;
        new_route.append(child) catch unreachable;
        num_routes += bfs2(G, alloc, new_route);
    }

    return num_routes;
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

    var graph = Graph.init(alloc);
    defer graph.deinit();

    var buf: [16]u8 = undefined;
    while (try input.reader().readUntilDelimiterOrEof(&buf, '\n')) |line| {
        var it = std.mem.split(u8, line, "-");
        var a = dup(alloc, it.next().?);
        var b = dup(alloc, it.next().?);

        if (!std.mem.eql(u8, b, "start") and !std.mem.eql(u8, a, "end")) {
            var a_gop = try graph.getOrPut(a);
            if (!a_gop.found_existing) {
                a_gop.value_ptr.* = std.ArrayList([]u8).init(alloc);
            }
            try a_gop.value_ptr.*.append(dup(alloc, b));
        }

        if (!std.mem.eql(u8, b, "end") and !std.mem.eql(u8, a, "start")) {
            var b_gop = try graph.getOrPut(b);
            if (!b_gop.found_existing) {
                b_gop.value_ptr.* = std.ArrayList([]u8).init(alloc);
            }
            try b_gop.value_ptr.*.append(dup(alloc, a));
        }
    }

    var route_start = std.ArrayList([]const u8).init(alloc);
    try route_start.append("start");
    try stdout.print("Number of routes with no duplicates: {}\n", .{bfs(&graph, alloc, route_start)});

    var route2_start = std.ArrayList([]const u8).init(alloc);
    try route2_start.append("start");
    try stdout.print("Number of routes with some duplicates: {}\n", .{bfs2(&graph, alloc, route2_start)});
}
