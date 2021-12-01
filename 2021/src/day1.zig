const std = @import("std");

pub fn main() anyerror!void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const alloc = &arena.allocator;

    var args_iter = std.process.args();
    _ = args_iter.skip();
    const input_file = args_iter.next(alloc).? catch unreachable;
    const file = std.fs.cwd().openFile(input_file, .{ .read = true }) catch |err| {
        std.log.err("Could not open {s} due to: {s}", .{ input_file, err });
        return;
    };
    defer file.close();

    var p1_count: u32 = 0;
    var p2_count: u32 = 0;
    var cur_window = [_]u32{ 0, 0, 0 };

    var buf: [16]u8 = undefined;
    while (try file.reader().readUntilDelimiterOrEof(&buf, '\n')) |line| {
        const next = try std.fmt.parseInt(u32, line, 0);

        p1_count += @boolToInt(next > cur_window[0] and cur_window[0] != 0); // Previous number

        std.mem.rotate(u32, cur_window[0..], 1); // Extremely basic windowing system
        p2_count += @boolToInt(next > cur_window[0] and cur_window[0] != 0); // Only need to check the differing value, not the actual sum
        cur_window[0] = next;
    }
    std.log.info("Number of increases: {}", .{p1_count});
    std.log.info("Number of 3-sum increases: {}", .{p2_count});
}
