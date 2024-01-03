const std = @import("std");

const Coord = struct {
    x: i32,
    y: i32,
};

const Probe = struct {
    velocity: Coord,
    pos: Coord,
};

fn is_within_target(tl: Coord, br: Coord, pos: Coord) bool {
    return pos.x >= tl.x and pos.x <= br.x and pos.y <= tl.y and pos.y >= br.y;
}

pub fn main() anyerror!void {
    const stdout = std.io.getStdOut().writer();

    var args_iter = std.process.args();
    _ = args_iter.skip(); // program name
    const input_file = args_iter.next() orelse unreachable;
    const input = std.fs.cwd().openFile(input_file, .{}) catch |err| {
        std.log.err("Could not open {s} due to: {}", .{ input_file, err });
        std.os.exit(1);
    };
    defer input.close();

    var target_tl = Coord{ .x = 0, .y = 0 };
    var target_br = Coord{ .x = 0, .y = 0 };

    var buf: [64]u8 = undefined;
    if (try input.reader().readUntilDelimiterOrEof(&buf, '\n')) |line| {
        // Parsing is hard when you haven't got a regex library
        var split = std.mem.split(u8, line[13..], ", "); // "target area: " == 13
        const x_vals = split.next().?;
        var x_split = std.mem.split(u8, x_vals[2..], "..");
        const x1 = try std.fmt.parseInt(i32, x_split.next().?, 0);
        const x2 = try std.fmt.parseInt(i32, x_split.next().?, 0);
        const y_vals = split.next().?;
        var y_split = std.mem.split(u8, y_vals[2..], "..");
        const y1 = try std.fmt.parseInt(i32, y_split.next().?, 0);
        const y2 = try std.fmt.parseInt(i32, y_split.next().?, 0);

        target_tl.x = std.math.min(x1, x2);
        target_br.x = std.math.max(x1, x2);
        target_tl.y = std.math.max(y1, y2);
        target_br.y = std.math.min(y1, y2);
    } else unreachable;

    var velocity_count: u32 = 0;

    var max_height_velo = Coord{ .x = 0, .y = 0 };
    var max_height: i32 = 0;

    // Arbitrarily use max_x of 2x max x coord
    // Arbitrarily use max_y of (abs) min y coord
    var velo_x: i32 = 0;
    while (velo_x < target_br.x * 2) : (velo_x += 1) {
        var velo_y: i32 = target_br.y;
        while (velo_y < try std.math.absInt(target_br.y)) : (velo_y += 1) {
            const initial_velocity = Coord{ .x = velo_x, .y = velo_y };
            var max_y: i32 = 0;
            var pos = Coord{ .x = 0, .y = 0 };
            var has_point_in_target = false;
            var velocity = initial_velocity;
            // As soon as we know we're never going to get within the target box, exit loop
            while (pos.y >= target_br.y and ((velocity.x >= 0 and pos.x <= target_br.x) or (velocity.x <= 0 and pos.x >= target_br.x))) {
                pos.x += velocity.x;
                pos.y += velocity.y;

                // drag
                if (velocity.x < 0) {
                    velocity.x += 1;
                } else if (velocity.x > 0) {
                    velocity.x -= 1;
                }
                velocity.y -= 1; // gravity

                max_y = std.math.max(max_y, pos.y);

                if (is_within_target(target_tl, target_br, pos)) {
                    has_point_in_target = true;
                    //std.debug.print("initial velocity {} with point within target: {any}, max_y: {}\n", .{ initial_velocity, pos, max_y });
                    break;
                }
            }
            if (has_point_in_target) {
                //std.debug.print("initial velocity within target: {}\n", .{initial_velocity});
                velocity_count += 1;
                if (max_y > max_height) {
                    max_height = max_y;
                    max_height_velo = initial_velocity;
                }
            }
        }
    }

    try stdout.print("Probe with initial velocity x={},y={} reaches y={}\n", .{ max_height_velo.x, max_height_velo.y, max_height });
    try stdout.print("Number of probe velocities where it ends up within target: {}\n", .{velocity_count});
}
