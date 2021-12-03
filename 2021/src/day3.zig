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

    var bit_counts = [1]u32{0} ** 12;
    var input_length: u32 = 0;
    var num_bits: usize = 0;

    var buf: [16]u8 = undefined;
    while (try input.reader().readUntilDelimiterOrEof(&buf, '\n')) |line| {
        num_bits = line.len;
        var i: usize = 0;
        while (i < num_bits) : (i += 1) {
            if (line[i] == '0') {
                // skip
            } else if (line[i] == '1') {
                bit_counts[i] += 1;
            } else {
                unreachable;
            }
        }
        input_length += 1;
    }
    std.debug.print("{any}\n", .{bit_counts});

    var gamma_rate: u32 = 0;
    var epsilon_rate: u32 = 0;

    var i: u32 = 0;
    while (i < num_bits) : (i += 1) {
        if (bit_counts[i] < input_length / 2) {
            epsilon_rate += std.math.pow(u32, 2, i);
        } else {
            gamma_rate += std.math.pow(u32, 2, i);
        }
    }

    try stdout.print("Gamma & Epsilon rates: {},{} ({})\n", .{ gamma_rate, epsilon_rate, gamma_rate * epsilon_rate });
}
