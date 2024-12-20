const std = @import("std");

const Dir = enum {
    down,
    up,
    forward,
};

pub fn main() anyerror!void {
    const stdout = std.io.getStdOut().writer();

    var args_iter = std.process.args();
    _ = args_iter.skip(); // program name
    const input_file = args_iter.next() orelse unreachable;
    const input = std.fs.cwd().openFile(input_file, .{}) catch |err| {
        std.log.err("Could not open {s} due to: {}", .{ input_file, err });
        std.process.exit(1);
    };
    defer input.close();

    var depth: u32 = 0;
    var pos: u32 = 0;

    var p2_depth: u32 = 0;
    var p2_pos: u32 = 0;
    var p2_aim: u32 = 0;

    var buf: [16]u8 = undefined;
    while (try input.reader().readUntilDelimiterOrEof(&buf, '\n')) |line| {
        var it = std.mem.split(u8, line, " ");
        const dir = std.meta.stringToEnum(Dir, it.next().?).?;
        const val = try std.fmt.parseInt(u32, it.next().?, 0);
        switch (dir) {
            Dir.down => {
                depth += val;
                p2_aim += val;
            },
            Dir.up => {
                depth -= val;
                p2_aim -= val;
            },
            Dir.forward => {
                pos += val;
                p2_pos += val;
                p2_depth += val * p2_aim;
            },
        }
    }

    try stdout.print("Final position: {},{} ({})\n", .{ depth, pos, depth * pos });
    try stdout.print("Final aimed position: {},{} ({})\n", .{ p2_depth, p2_pos, p2_depth * p2_pos });
}
