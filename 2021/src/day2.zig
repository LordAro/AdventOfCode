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

    var depth: u32 = 0;
    var pos: u32 = 0;

    var p2_depth: u32 = 0;
    var p2_pos: u32 = 0;
    var p2_aim: u32 = 0;

    var buf: [16]u8 = undefined;
    while (try input.reader().readUntilDelimiterOrEof(&buf, '\n')) |line| {
        var it = std.mem.split(line, " ");
        const dir = it.next().?;
        const val = try std.fmt.parseInt(u32, it.next().?, 0);
        if (std.mem.eql(u8, dir, "down")) {
            depth += val;
            p2_aim += val;
        } else if (std.mem.eql(u8, dir, "up")) {
            depth -= val;
            p2_aim -= val;
        } else if (std.mem.eql(u8, dir, "forward")) {
            pos += val;
            p2_pos += val;
            p2_depth += val * p2_aim;
        }
    }

    try stdout.print("Final position: {},{} ({})\n", .{ depth, pos, depth * pos });
    try stdout.print("Final aimed position: {},{} ({})\n", .{ p2_depth, p2_pos, p2_depth * p2_pos });
}
