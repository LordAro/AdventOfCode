const std = @import("std");

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

    var lanterns = std.ArrayList(u8).init(alloc);
    defer lanterns.deinit();

    var buf: [4]u8 = undefined;
    while (try input.reader().readUntilDelimiterOrEof(&buf, ',')) |chr| {
        try lanterns.append(try std.fmt.charToDigit(chr[0], 10));
    }

    var day: u32 = 1;
    while (day <= 80) : (day += 1) {
        var newCount: u32 = 0;
        for (lanterns.items) |*l| {
            if (l.* == 0) {
                newCount += 1;
                l.* = 6;
            } else {
                l.* -= 1;
            }
        }
        while (newCount > 0) : (newCount -= 1) {
            try lanterns.append(8);
        }
    }

    try stdout.print("Lanterns after 80 days: {}\n", .{lanterns.items.len});
}
