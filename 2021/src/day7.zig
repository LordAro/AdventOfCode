const std = @import("std");

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

    var buf: [8]u8 = undefined;
    while (try input.reader().readUntilDelimiterOrEof(&buf, ',')) |line| {
        const line2 = std.mem.trimRight(u8, line, "\n");
        const n = try std.fmt.parseInt(u32, line2, 0);
        try numbers.append(n);
    }

    var min: u32 = std.math.maxInt(u32);
    var max: u32 = 0;
    for (numbers.items) |n| {
        min = @min(min, n);
        max = @max(max, n);
    }

    var minFuelCost: u32 = std.math.maxInt(u32);
    var minFuelCost2: u32 = std.math.maxInt(u32);
    var pos = min;
    while (pos <= max) : (pos += 1) {
        var fuelCost: u32 = 0;
        var fuelCost2: u32 = 0;
        for (numbers.items) |n| {
            const distance = @as(u32, @intCast(try std.math.absInt(@as(i32, @intCast(n)) - @as(i32, @intCast(pos)))));
            fuelCost += distance;
            fuelCost2 += distance * (distance + 1) / 2; // triangle numbers
        }
        minFuelCost = @min(minFuelCost, fuelCost);
        minFuelCost2 = @min(minFuelCost2, fuelCost2);
    }

    try stdout.print("Min fuel cost (constant rate): {}\n", .{minFuelCost});
    try stdout.print("Min fuel cost (variable rate): {}\n", .{minFuelCost2});
}
