const std = @import("std");

fn step_grid(grid: [10][10]u8, flash_count: *u32) [10][10]u8 {
    var new_grid = grid;
    // increase all by 1
    for (grid) |r, j| {
        for (r) |_, i| {
            new_grid[j][i] += 1;
        }
    }

    // while octopus with energy level > 9, flash
    while (true) {
        var local_flash_count: u32 = 0;
        for (new_grid) |r, j| {
            for (r) |_, i| {
                if (new_grid[j][i] > 9) {
                    local_flash_count += 1;
                    new_grid[j][i] = 0;

                    // increase adjacents
                    var adj_j: isize = -1;
                    while (adj_j <= 1) : (adj_j += 1) {
                        const new_j: isize = @intCast(isize, j) + adj_j;
                        if (new_j < 0 or new_j > 9) continue;

                        var adj_i: isize = -1;
                        while (adj_i <= 1) : (adj_i += 1) {
                            const new_i: isize = @intCast(isize, i) + adj_i;
                            if (new_i < 0 or new_i > 9) continue;

                            if (new_grid[@intCast(usize, new_j)][@intCast(usize, new_i)] == 0) continue; // already flashed, don't reincrement (also covers ourself)
                            new_grid[@intCast(usize, new_j)][@intCast(usize, new_i)] += 1;
                        }
                    }
                }
            }
        }
        flash_count.* += local_flash_count;
        if (local_flash_count == 0) break; // nothing more to do
    }

    return new_grid;
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

    var grid = [_][10]u8{[_]u8{0} ** 10} ** 10;

    var i: usize = 0;
    var buf: [16]u8 = undefined;
    while (try input.reader().readUntilDelimiterOrEof(&buf, '\n')) |line| : (i += 1) {
        for (line) |c, j| {
            grid[i][j] = try std.fmt.charToDigit(c, 10);
        }
    }

    var flash_count: u32 = 0;
    var flash_count100: u32 = 0;

    var all_flash = false;
    var step_n: u32 = 0;
    while (!all_flash) : (step_n += 1) {
        grid = step_grid(grid, &flash_count);
        var grid_sum: u32 = 0;
        for (grid) |r, jx| {
            for (r) |_, ix| {
                grid_sum += grid[jx][ix];
            }
        }
        if (grid_sum == 0) all_flash = true;
        if (step_n == 99) flash_count100 = flash_count;
    }

    try stdout.print("Number of flashes after 100 steps: {}\n", .{flash_count100});
    // don't need to +1 here as it gets incremented by the loop prior to exiting
    try stdout.print("Step count for all flashing: {}\n", .{step_n});
}
