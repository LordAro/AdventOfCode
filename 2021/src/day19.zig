const std = @import("std");

const Coord = struct {
    x: i32,
    y: i32,
    z: i32,

    fn add(a: Coord, b: Coord) Coord {
        return Coord{ .x = a.x + b.x, .y = a.y + b.y, .z = a.z + b.z };
    }

    fn sub(a: Coord, b: Coord) Coord {
        return Coord{ .x = a.x - b.x, .y = a.y - b.y, .z = a.z - b.z };
    }
};

inline fn rot_x(c: Coord) Coord {
    return Coord{ .x = c.x, .y = -c.z, .z = c.y };
}

inline fn rot_y(c: Coord) Coord {
    return Coord{ .x = c.z, .y = c.y, .z = -c.x };
}

inline fn rot_z(c: Coord) Coord {
    return Coord{ .x = c.y, .y = -c.x, .z = c.z };
}

inline fn rotate_coord(c: Coord, n: u32) Coord {
    // lookup table stolen from https://stackoverflow.com/a/50546727
    return switch (n) {
        0 => c, // I
        1 => rot_x(c), // X = YXZ
        2 => rot_y(c), // Y = ZYX
        3 => rot_z(c), // Z = XZY
        4 => rot_x(rot_x(c)), // XX = XYXZ = YXXY = YXYZ = YXZX = YYZZ = YZXZ = ZXXZ = ZZYY
        5 => rot_y(rot_x(c)), // XY = YZ = ZX = XZYX = YXZY = ZYXZ
        6 => rot_z(rot_x(c)), // XZ = XXZY = YXZZ = YYYX = ZYYY
        7 => rot_x(rot_y(c)), // YX = XZZZ = YYXZ = ZYXX = ZZZY
        8 => rot_y(rot_y(c)), // YY = XXZZ = XYYX = YZYX = ZXYX = ZYXY = ZYYZ = ZYZX = ZZXX
        9 => rot_y(rot_z(c)), // ZY = XXXZ = XZYY = YXXX = ZZYX
        10 => rot_z(rot_z(c)), // ZZ = XXYY = XYZY = XZXY = XZYZ = XZZX = YYXX = YZZY = ZXZY
        11 => rot_x(rot_x(rot_x(c))), // XXX
        12 => rot_y(rot_x(rot_x(c))), // XXY = XYZ = XZX = YZZ = ZXZ
        13 => rot_z(rot_x(rot_x(c))), // XXZ = ZYY
        14 => rot_x(rot_y(rot_x(c))), // XYX = YXY = YYZ = YZX = ZXX
        15 => rot_y(rot_y(rot_x(c))), // XYY = YZY = ZXY = ZYZ = ZZX
        16 => rot_z(rot_z(rot_x(c))), // XZZ = YYX
        17 => rot_x(rot_x(rot_y(c))), // YXX = ZZY
        18 => rot_y(rot_y(rot_y(c))), // YYY
        19 => rot_z(rot_z(rot_z(c))), // ZZZ
        20 => rot_y(rot_x(rot_x(rot_x(c)))), // XXXY = XXYZ = XXZX = XYZZ = XZXZ = YZZZ = ZXZZ = ZYYX
        21 => rot_x(rot_y(rot_x(rot_x(c)))), // XXYX = XYXY = XYYZ = XYZX = XZXX = YXYY = YYZY = YZXY = YZYZ = YZZX = ZXXY = ZXYZ = ZXZX = ZYZZ = ZZXZ
        22 => rot_x(rot_x(rot_y(rot_x(c)))), // XYXX = XZZY = YXYX = YYXY = YYYZ = YYZX = YZXX = ZXXX
        23 => rot_y(rot_y(rot_y(rot_x(c)))), // XYYY = YXXZ = YZYY = ZXYY = ZYZY = ZZXY = ZZYZ = ZZZX
        else => unreachable,
    };
}

pub fn main() !void {
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

    var scanner_outputs = std.ArrayList(std.ArrayList(Coord)).init(alloc);
    defer scanner_outputs.deinit();

    var scanner_output = std.ArrayList(Coord).init(alloc);

    var buf: [128]u8 = undefined;
    while (try input.reader().readUntilDelimiterOrEof(&buf, '\n')) |line| {
        if (line.len == 0) {
            try scanner_outputs.append(scanner_output);
        } else if (std.mem.startsWith(u8, line, "--- scanner")) {
            scanner_output = std.ArrayList(Coord).init(alloc);
        } else {
            var it = std.mem.split(u8, line, ",");
            const x = try std.fmt.parseInt(i32, it.next().?, 0);
            const y = try std.fmt.parseInt(i32, it.next().?, 0);
            const z = try std.fmt.parseInt(i32, it.next().?, 0);
            try scanner_output.append(Coord{ .x = x, .y = y, .z = z });
        }
    }
    try scanner_outputs.append(scanner_output); // last one

    var absolute_beacons = std.AutoHashMap(Coord, void).init(alloc);
    defer absolute_beacons.deinit();

    var scanner_positions = std.ArrayList(Coord).init(alloc);
    defer scanner_positions.deinit();

    const initial_scanner_ix: usize = 0;
    for (scanner_outputs.items[initial_scanner_ix].items) |c| {
        try absolute_beacons.put(c, {});
    }
    try scanner_positions.append(Coord{ .x = 0, .y = 0, .z = 0 });

    scanner_outputs.items[initial_scanner_ix].deinit();
    _ = scanner_outputs.swapRemove(initial_scanner_ix);

    while (scanner_outputs.items.len != 0) {
        outer: for (scanner_outputs.items, 0..) |relative_beacons, beacon_number| {
            var abs_beacon_it = absolute_beacons.keyIterator();
            while (abs_beacon_it.next()) |abs_beacon| {
                for (relative_beacons.items) |initial_relative_beacon| {
                    var rot_n: u32 = 0;
                    while (rot_n < 24) : (rot_n += 1) {
                        const beacon_offset = abs_beacon.sub(rotate_coord(initial_relative_beacon, rot_n));
                        var count_matching: u32 = 0;
                        for (relative_beacons.items) |relative_beacon| {
                            const possible_abs_beacon = rotate_coord(relative_beacon, rot_n).add(beacon_offset);
                            if (absolute_beacons.contains(possible_abs_beacon)) {
                                count_matching += 1;
                            }
                            if (count_matching >= 12) break;
                        }
                        if (count_matching >= 12) {
                            //std.debug.print("rot: {} offset: {any}\n", .{ rot_n, beacon_offset });
                            for (relative_beacons.items) |relative_beacon| {
                                const new_abs_beacon = rotate_coord(relative_beacon, rot_n).add(beacon_offset);
                                try absolute_beacons.put(new_abs_beacon, {});
                            }
                            try scanner_positions.append(beacon_offset);
                            scanner_outputs.items[beacon_number].deinit();
                            _ = scanner_outputs.swapRemove(beacon_number);
                            break :outer;
                        }
                    }
                }
            }
        }
    }

    var max_manhattan_dist: i32 = 0;
    for (scanner_positions.items, 0..) |scanner_a, i| {
        for (scanner_positions.items[i + 1 ..]) |scanner_b| {
            const diff = scanner_a.sub(scanner_b);
            const manhattan = (try std.math.absInt(diff.x)) + (try std.math.absInt(diff.y)) + (try std.math.absInt(diff.z));
            max_manhattan_dist = @max(max_manhattan_dist, manhattan);
        }
    }

    try stdout.print("Number of beacons: {}\n", .{absolute_beacons.count()});
    try stdout.print("Maximum manhattan distance: {}\n", .{max_manhattan_dist});
}

test "rotate all combinations" {
    var alloc = std.testing.allocator;
    var rotations = std.AutoHashMap(Coord, void).init(alloc);
    defer rotations.deinit();
    var rot_n: u32 = 0;
    const c = Coord{ .x = 2, .y = 3, .z = 5 };
    while (rot_n < 24) : (rot_n += 1) {
        try rotations.put(rotate_coord(c, rot_n), {});
    }
    try std.testing.expectEqual(rotations.count(), 24);
}
