const std = @import("std");
//const rb = @import("./rb.zig");

const TreeNode = struct {
    allocator: *std.mem.Allocator,

    parent: ?*TreeNode,
    value: ?usize,
    left_child: ?*TreeNode,
    right_child: ?*TreeNode,

    fn init(alloc: *std.mem.Allocator) !*TreeNode {
        var node = try alloc.create(TreeNode);
        node.* = TreeNode{
            .allocator = alloc,
            .parent = null,
            .value = null,
            .left_child = null,
            .right_child = null,
        };
        return node;
    }

    fn deinit(tree: *const TreeNode) void {
        if (tree.left_child) |child| {
            child.deinit();
        }
        if (tree.right_child) |child| {
            child.deinit();
        }
        tree.allocator.destroy(tree);
    }

    fn copy(tree: *const TreeNode) std.mem.Allocator.Error!*TreeNode {
        var new = try TreeNode.init(tree.allocator);
        new.value = tree.value;
        if (tree.left_child) |child| {
            const cp = try child.copy();
            new.add_left_child(cp);
        }
        if (tree.right_child) |child| {
            const cp = try child.copy();
            new.add_right_child(cp);
        }
        return new;
    }

    pub fn next(constnode: *TreeNode) ?*TreeNode {
        var node = constnode;

        if (node.right_child) |right| {
            var n = right;
            while (n.left_child) |left|
                n = left;
            return n;
        }

        while (true) {
            var parent = node.parent;
            if (parent) |p| {
                if (node != p.right_child)
                    return p;
                node = p;
            } else return null;
        }
    }

    pub fn prev(constnode: *TreeNode) ?*TreeNode {
        var node = constnode;

        if (node.left_child) |left| {
            var n = left;
            while (n.right_child) |right|
                n = right;
            return n;
        }

        while (true) {
            var parent = node.parent;
            if (parent) |p| {
                if (node != p.left_child)
                    return p;
                node = p;
            } else return null;
        }
    }

    fn prevval(constnode: *TreeNode) ?*TreeNode {
        var node: ?*TreeNode = constnode.prev();
        while (node) |n| : (node = n.prev()) {
            if (n.value != null) {
                return n;
            }
        }
        return null;
    }

    fn nextval(constnode: *TreeNode) ?*TreeNode {
        var node: ?*TreeNode = constnode.next();
        while (node) |n| : (node = n.next()) {
            if (n.value != null) {
                return n;
            }
        }
        return null;
    }

    fn add_left_child(tree: *TreeNode, c: ?*TreeNode) void {
        tree.left_child = c;
        tree.left_child.?.parent = tree;
    }

    fn add_right_child(tree: *TreeNode, c: ?*TreeNode) void {
        tree.right_child = c;
        tree.right_child.?.parent = tree;
    }

    fn magnitude(tree: *TreeNode) usize {
        const left_mag = 3 * (tree.left_child.?.value orelse tree.left_child.?.magnitude());
        const right_mag = 2 * (tree.right_child.?.value orelse tree.right_child.?.magnitude());
        return left_mag + right_mag;
    }

    // Allows us to use {any} in format strings
    pub fn format(
        tree: TreeNode,
        comptime fmt: []const u8,
        options: std.fmt.FormatOptions,
        out_stream: anytype,
    ) @TypeOf(out_stream).Error!void {
        if (tree.value) |v| {
            try std.fmt.format(out_stream, "{}", .{v});
        } else {
            try out_stream.writeAll("[");
            try tree.left_child.?.format(fmt, options, out_stream);
            try out_stream.writeAll(",");
            try tree.right_child.?.format(fmt, options, out_stream);
            try out_stream.writeAll("]");
        }
    }
};

const ParseError = std.mem.Allocator.Error || std.fmt.ParseIntError;

fn parse_snail(alloc: *std.mem.Allocator, line: []const u8) ParseError!*TreeNode {
    var root = try TreeNode.init(alloc);
    if (line[0] == '[') {
        var depth: usize = 0;
        var comma_pos: usize = 0;
        var i: usize = 0;
        while (i < line.len) : (i += 1) {
            if (line[i] == '[') {
                depth += 1;
            } else if (line[i] == ']') {
                depth -= 1;
            }
            if (depth == 1 and line[i] == ',') {
                // Can technically stop parsing at this point, but it's nicer to keep going and confirm input is valid
                comma_pos = i;
            }
        }
        std.debug.assert(line[0] == '[' and line[line.len - 1] == ']');
        std.debug.assert(comma_pos != 0);
        std.debug.assert(depth == 0);

        const left_child = try parse_snail(alloc, line[1..comma_pos]);
        const right_child = try parse_snail(alloc, line[comma_pos + 1 .. line.len - 1]);
        root.add_left_child(left_child);
        root.add_right_child(right_child);
    } else {
        root.value = try std.fmt.parseInt(u8, line, 0);
    }
    return root;
}

fn find_deep(tree: ?*TreeNode, depth: usize, target: usize) ?*TreeNode {
    if (tree.?.value != null) {
        if (depth == target) {
            return tree.?.parent;
        }
        return null;
    }

    return find_deep(tree.?.left_child, depth + 1, target) orelse find_deep(tree.?.right_child, depth + 1, target);
}

fn find_big(tree: ?*TreeNode) ?*TreeNode {
    if (tree.?.value != null) {
        if (tree.?.value.? >= 10) {
            return tree;
        }
        return null;
    }

    return find_big(tree.?.left_child) orelse find_big(tree.?.right_child);
}

fn explode_node(node: *TreeNode) !void {
    std.debug.assert(node.left_child.?.value != null);
    const prev_node = node.left_child.?.prevval();
    if (prev_node) |n| {
        n.value.? += node.left_child.?.value.?;
    } else {
        node.left_child.?.value = 0;
    }

    std.debug.assert(node.right_child.?.value != null);
    const next_node = node.right_child.?.nextval();
    if (next_node) |n| {
        n.value.? += node.right_child.?.value.?;
    } else {
        node.right_child.?.value = 0;
    }

    const new_node = try TreeNode.init(node.allocator);
    new_node.value = 0;
    if (node.parent.?.left_child == node) {
        node.parent.?.add_left_child(new_node);
    } else {
        node.parent.?.add_right_child(new_node);
    }
    node.deinit();
}

fn explode_tree(tree: *TreeNode) !bool {
    var exploded = false;
    const deep_node = find_deep(tree, 0, 5);
    if (deep_node) |n| {
        //std.debug.print("ex: t: {any} d: {any}\n", .{ tree, n });
        try explode_node(n);
    }
    return deep_node != null;
}

fn split_node(node: *TreeNode) !void {
    std.debug.assert(node.value != null);
    const lv = node.value.? / 2; // floor
    const rv = (node.value.? + 1) / 2; // ceil

    const lnode = try TreeNode.init(node.allocator);
    const rnode = try TreeNode.init(node.allocator);
    lnode.value = lv;
    rnode.value = rv;
    node.value = null;
    node.add_left_child(lnode);
    node.add_right_child(rnode);
}

fn split_tree(tree: *TreeNode) !bool {
    const big_node = find_big(tree);
    if (big_node) |n| {
        //std.debug.print("sp: t: {any} d: {any}\n", .{ tree, n });
        try split_node(n);
    }
    return big_node != null;
}

fn add_snail(alloc: *std.mem.Allocator, left: *TreeNode, right: *TreeNode) !*TreeNode {
    var root = try TreeNode.init(alloc);
    root.add_left_child(try left.copy());
    root.add_right_child(try right.copy());

    while ((try explode_tree(root)) or (try split_tree(root))) {}
    return root;
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

    var snail_numbers = std.ArrayList(*TreeNode).init(alloc);

    var snail_sum: ?*TreeNode = null;
    defer snail_sum.?.deinit();

    var buf: [128]u8 = undefined;
    while (try input.reader().readUntilDelimiterOrEof(&buf, '\n')) |line| {
        const s = try parse_snail(alloc, line);
        if (snail_sum) |sum| {
            const new_snail_sum = try add_snail(alloc, sum, s);
            sum.deinit();
            snail_sum = new_snail_sum;
        } else {
            snail_sum = s;
        }
        try snail_numbers.append(s);
        //std.debug.print("{any}\n", .{s});
    }

    var max_magnitude: usize = 0;

    var i: usize = 0;
    while (i < snail_numbers.items.len) : (i += 1) {
        var j: usize = i + 1;
        while (j < snail_numbers.items.len) : (j += 1) {
            const sum1 = try add_snail(alloc, snail_numbers.items[i], snail_numbers.items[j]);
            defer sum1.deinit();
            max_magnitude = std.math.max(max_magnitude, sum1.magnitude());

            const sum2 = try add_snail(alloc, snail_numbers.items[j], snail_numbers.items[i]);
            defer sum2.deinit();
            max_magnitude = std.math.max(max_magnitude, sum2.magnitude());
        }
    }
    try stdout.print("Total sum snail number: {any}, magnitude: {}\n", .{ snail_sum, snail_sum.?.magnitude() });
    try stdout.print("Maximum sum magnitude from any pair: {}\n", .{max_magnitude});
}

test "parse snail" {
    var alloc = std.testing.allocator;
    const inputs = [_][]const u8{
        "[1,2]",
        "[[1,2],3]",
        "[9,[8,7]]",
        "[[1,9],[8,5]]",
        "[[[[1,2],[3,4]],[[5,6],[7,8]]],9]",
        "[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]",
        "[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]",
    };
    for (inputs) |input| {
        const s = try parse_snail(alloc, input);
        defer s.deinit();
        try std.testing.expectFmt(input, "{any}", .{s});
    }
}

test "find deep" {
    var alloc = std.testing.allocator;
    const s = try parse_snail(alloc, "[[[[[9,8],1],2],3],4]");
    defer s.deinit();
    const deep = find_deep(s, 0, 5);
    try std.testing.expect(deep != null);
    try std.testing.expect(deep.?.value == null);
    try std.testing.expectEqual(deep.?.left_child.?.value, 9);
    try std.testing.expectEqual(deep.?.right_child.?.value, 8);
}

test "find deep 2" {
    var alloc = std.testing.allocator;
    const s = try parse_snail(alloc, "[7,[6,[5,[4,[3,2]]]]]");
    defer s.deinit();
    const deep = find_deep(s, 0, 5);
    try std.testing.expect(deep != null);
    try std.testing.expect(deep.?.value == null);
    try std.testing.expectEqual(deep.?.left_child.?.value, 3);
    try std.testing.expectEqual(deep.?.right_child.?.value, 2);
}

test "prev next 1" {
    var alloc = std.testing.allocator;
    const s = try parse_snail(alloc, "[[[[[9,8],1],2],3],4]");
    defer s.deinit();
    const deep = find_deep(s, 0, 5);
    const p = deep.?.left_child.?.prevval();
    const n = deep.?.right_child.?.nextval();
    try std.testing.expect(p == null);
    try std.testing.expect(n != null);
    try std.testing.expect(n.?.value != null);
    try std.testing.expectEqual(n.?.value, 1);
}

test "prev next 2" {
    var alloc = std.testing.allocator;
    const s = try parse_snail(alloc, "[7,[6,[5,[4,[3,2]]]]]");
    defer s.deinit();
    const deep = find_deep(s, 0, 5);
    const p = deep.?.left_child.?.prevval();
    const n = deep.?.right_child.?.nextval();
    try std.testing.expect(n == null);
    try std.testing.expect(p != null);
    try std.testing.expect(p.?.value != null);
    try std.testing.expectEqual(p.?.value, 4);
}

test "prev next 3" {
    var alloc = std.testing.allocator;
    const s = try parse_snail(alloc, "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
    defer s.deinit();
    const deep = find_deep(s, 0, 5);
    const p = deep.?.left_child.?.prevval();
    const n = deep.?.right_child.?.nextval();
    try std.testing.expect(p != null);
    try std.testing.expect(p.?.value != null);
    try std.testing.expectEqual(p.?.value, 1);
    try std.testing.expect(n != null);
    try std.testing.expect(n.?.value != null);
    try std.testing.expectEqual(n.?.value, 6);
}

test "explode tree" {
    var alloc = std.testing.allocator;
    const s = try parse_snail(alloc, "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
    defer s.deinit();
    const exploded = try explode_tree(s);
    try std.testing.expect(exploded);
    try std.testing.expectFmt("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]", "{any}", .{s});
    const exploded2 = try explode_tree(s);
    try std.testing.expect(exploded2);
    try std.testing.expectFmt("[[3,[2,[8,0]]],[9,[5,[7,0]]]]", "{any}", .{s});
}

test "add node 1" {
    var alloc = std.testing.allocator;
    const s1 = try parse_snail(alloc, "[[[[4,3],4],4],[7,[[8,4],9]]]");
    defer s1.deinit();
    const s2 = try parse_snail(alloc, "[1,1]");
    defer s2.deinit();

    const new_root = try add_snail(alloc, s1, s2);
    defer new_root.deinit();

    try std.testing.expectFmt("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", "{any}", .{new_root});
}
