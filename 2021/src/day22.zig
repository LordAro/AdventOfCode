const std = @import("std");

const Coord = struct {
    x: i32,
    y: i32,
    z: i32,
};

const Cube = struct {
    p1: Coord,
    p2: Coord,
};

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

    // arrayhashmap preserves iteration order so we can just iterate through it
    // if we could iterate backwards we could exit early, but we can't :(
    var cubes = std.AutoArrayHashMap(Cube, bool).init(alloc);

    var input_reader = input.reader();
    var buf: [1024]u8 = undefined;
    while (try input_reader.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        const is_on = std.mem.indexOf(u8, line, "on") == 0;
        var first_space = std.mem.indexOf(u8, line, " ") orelse unreachable;

        var axis_split = std.mem.split(u8, line[first_space + 1 ..], ",");

        const x_vals = axis_split.next().?;
        var x_split = std.mem.split(u8, x_vals[2..], "..");
        const x1 = try std.fmt.parseInt(i32, x_split.next().?, 0);
        const x2 = try std.fmt.parseInt(i32, x_split.next().?, 0);

        const y_vals = axis_split.next().?;
        var y_split = std.mem.split(u8, y_vals[2..], "..");
        const y1 = try std.fmt.parseInt(i32, y_split.next().?, 0);
        const y2 = try std.fmt.parseInt(i32, y_split.next().?, 0);

        const z_vals = axis_split.next().?;
        var z_split = std.mem.split(u8, z_vals[2..], "..");
        const z1 = try std.fmt.parseInt(i32, z_split.next().?, 0);
        const z2 = try std.fmt.parseInt(i32, z_split.next().?, 0);

        const p1: Coord = .{ .x = x1, .y = y1, .z = z1 };
        const p2: Coord = .{ .x = x2, .y = y2, .z = z2 };
        cubes.put(Cube{ .p1 = p1, .p2 = p2 }, is_on) catch unreachable;
    }

    var cube_count: usize = 0;
    var x: i32 = -50;
    while (x <= 50) : (x += 1) {
        var y: i32 = -50;
        while (y <= 50) : (y += 1) {
            var z: i32 = -50;
            while (z <= 50) : (z += 1) {
                const c = Coord{ .x = x, .y = y, .z = z };

                var is_illuminated = false;
                var cube_it = cubes.iterator();
                while (cube_it.next()) |cube_obj| {
                    const cube = cube_obj.key_ptr;
                    if (cube.p1.x <= c.x and c.x <= cube.p2.x and cube.p1.y <= c.y and c.y <= cube.p2.y and cube.p1.z <= c.z and c.z <= cube.p2.z) {
                        is_illuminated = cube_obj.value_ptr.*;
                    }
                }
                if (is_illuminated) {
                    cube_count += 1;
                }
            }
        }
    }
    try stdout.print("Cubes turned on: {}", .{cube_count});
}
