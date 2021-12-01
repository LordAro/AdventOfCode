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
    var count: u32 = 0;
    var previous: u32 = std.math.maxInt(u32);

    var sum_count: u32 = 0;
    var cur_window = [_]u32{ 0, 0, 0 };
    var prev_sum: u32 = std.math.maxInt(u32);

    var buf: [16]u8 = undefined;
    while (try file.reader().readUntilDelimiterOrEof(&buf, '\n')) |line| {
        const next = try std.fmt.parseInt(u32, line, 0);
        if (next > previous) count += 1;

        std.mem.rotate(u32, cur_window[0..], 1); // Extremely basic windowing system
        cur_window[0] = next;
        if (cur_window[0] != 0 and cur_window[1] != 0 and cur_window[2] != 0) {
            const new_sum = cur_window[0] + cur_window[1] + cur_window[2];
            if (new_sum > prev_sum) sum_count += 1;
            prev_sum = new_sum;
        }
        previous = next;
    }
    std.log.info("Number of increases: {}", .{count});
    std.log.info("Number of 3-sum increases: {}", .{sum_count});
}
