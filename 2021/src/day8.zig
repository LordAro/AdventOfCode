const std = @import("std");

fn findItemWithCb(arr: anytype, foundItems: anytype, callback: anytype) []const u8 {
    for (arr.items) |str| {
        if (callback(str, foundItems)) {
            return str;
        }
    }
    unreachable;
}

fn contains(a: []const u8, b: []const u8) bool {
    for (b) |c| {
        if (std.mem.indexOfScalar(u8, a, c) == null) {
            return false;
        }
    }
    return true;
}

// Unique lengths
inline fn is_1(s: []const u8, foundItems: anytype) bool {
    _ = foundItems;
    return s.len == 2;
}
inline fn is_4(s: []const u8, foundItems: anytype) bool {
    _ = foundItems;
    return s.len == 4;
}
inline fn is_7(s: []const u8, foundItems: anytype) bool {
    _ = foundItems;
    return s.len == 3;
}
inline fn is_8(s: []const u8, foundItems: anytype) bool {
    _ = foundItems;
    return s.len == 7;
}

// 0 is the only 6-length that contains 7 and does not contain 9
inline fn is_0(s: []const u8, foundItems: anytype) bool {
    return s.len == 6 and contains(s, foundItems[7]) and !contains(s, foundItems[9]);
}

// 2 is the only 5-length that is not 5 and is not 3
inline fn is_2(s: []const u8, foundItems: anytype) bool {
    return s.len == 5 and !contains(s, foundItems[5]) and !contains(s, foundItems[3]);
}

// 3 is the only 5-length that contains 7
inline fn is_3(s: []const u8, foundItems: anytype) bool {
    return s.len == 5 and contains(s, foundItems[7]);
}

// 5 is the only 5-length that is *contained by* 6
inline fn is_5(s: []const u8, foundItems: anytype) bool {
    return s.len == 5 and contains(foundItems[6], s);
}

// 6 is the only 6-length that does not contain 0 and does not contain 9
inline fn is_6(s: []const u8, foundItems: anytype) bool {
    return s.len == 6 and !contains(s, foundItems[0]) and !contains(s, foundItems[9]);
}

// 9 is the only 6-length that contains 4
inline fn is_9(s: []const u8, foundItems: anytype) bool {
    return s.len == 6 and contains(s, foundItems[4]);
}

fn decodeSegments(alloc: std.mem.Allocator, line: []const u8) anyerror![4]u32 {
    var line_it = std.mem.split(u8, line, " | ");

    // Can't find any better way of converting an iterator into an array
    var input_values_it = std.mem.split(u8, line_it.next().?, " ");
    var input_values = std.ArrayList([]const u8).init(alloc);
    while (input_values_it.next()) |str| {
        try input_values.append(str);
    }
    defer input_values.deinit();

    var output_values_it = std.mem.split(u8, line_it.next().?, " ");
    var output_values = std.ArrayList([]const u8).init(alloc);
    defer output_values.deinit();
    while (output_values_it.next()) |str| {
        try output_values.append(str);
    }

    // dirty initialisation, but i can't find a better way of doing it
    var foundItems: [10][]const u8 = [_][]const u8{""} ** 10;
    foundItems[1] = findItemWithCb(input_values, foundItems, is_1);
    foundItems[4] = findItemWithCb(input_values, foundItems, is_4);
    foundItems[7] = findItemWithCb(input_values, foundItems, is_7);
    foundItems[8] = findItemWithCb(input_values, foundItems, is_8);

    foundItems[3] = findItemWithCb(input_values, foundItems, is_3);

    foundItems[9] = findItemWithCb(input_values, foundItems, is_9);

    foundItems[0] = findItemWithCb(input_values, foundItems, is_0);

    foundItems[6] = findItemWithCb(input_values, foundItems, is_6);

    foundItems[5] = findItemWithCb(input_values, foundItems, is_5);

    foundItems[2] = findItemWithCb(input_values, foundItems, is_2);

    //std.debug.print("{s}\n", .{foundItems});

    var ret: [4]u32 = [_]u32{ 0, 0, 0, 0 };
    for (output_values.items, &ret) |val, *output| {
        for (foundItems, 0..) |n, i| {
            if (val.len == n.len and contains(val, n) and contains(n, val)) {
                output.* = @as(u32, @intCast(i));
                break;
            }
        }
    }
    return ret;
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

    var numbers = std.ArrayList(u32).init(alloc);
    defer numbers.deinit();

    var count1478: u32 = 0;
    var output_sum: u32 = 0;

    var buf: [100]u8 = undefined;
    while (try input.reader().readUntilDelimiterOrEof(&buf, '\n')) |line| {
        const ret = try decodeSegments(alloc, line);
        for (ret) |i| {
            if (i == 1 or i == 4 or i == 7 or i == 8) {
                count1478 += 1;
            }
        }
        output_sum += ret[0] * 1000 + ret[1] * 100 + ret[2] * 10 + ret[3];
    }

    try stdout.print("Total number of 1,4,7,8: {}\n", .{count1478});
    try stdout.print("Total output sum: {}\n", .{output_sum});
}

test "example input" {
    const alloc = std.testing.allocator;
    const example_input =
        \\acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf
        \\be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
        \\edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
        \\fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
        \\fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
        \\aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
        \\fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
        \\dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
        \\bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
        \\egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
        \\gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
    ;
    const expected_output_nums = [_]u32{ 5353, 8394, 9781, 1197, 9361, 4873, 8418, 4548, 1625, 8717, 4315 };
    var example_it = std.mem.split(example_input, "\n");
    var i: usize = 0;
    while (example_it.next()) |line| : (i += 1) {
        const result = try decodeSegments(alloc, line);
        const result_n = result[0] * 1000 + result[1] * 100 + result[2] * 10 + result[3];
        try std.testing.expectEqual(result_n, expected_output_nums[i]);
    }
}
