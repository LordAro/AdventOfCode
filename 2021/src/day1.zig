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

    var lines = std.ArrayList(u32).init(alloc);
    defer lines.deinit();

    var buf: [16]u8 = undefined;
    while (try input.reader().readUntilDelimiterOrEof(&buf, '\n')) |line| {
        try lines.append(try std.fmt.parseInt(u32, line, 10));
    }

    var p1_count: u32 = 0;
    var p2_count: u32 = 0;
    var i: usize = 0;
    const len = lines.items.len;
    while (i < len) : (i += 1) {
        p1_count += @boolToInt(i < len - 1 and lines.items[i + 1] > lines.items[i]);
        p2_count += @boolToInt(i < len - 3 and lines.items[i + 3] > lines.items[i]);
    }

    try stdout.print("Number of increases: {}\n", .{p1_count});
    try stdout.print("Number of 3-sum increases: {}\n", .{p2_count});
}
