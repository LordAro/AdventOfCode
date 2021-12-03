const std = @import("std");

fn get_bitcount(arr: std.ArrayList(u16), i: u4) u16 {
    var bit_count: u16 = 0;
    for (arr.items) |dn| {
        if ((dn & (@as(u16, 1) << i)) != 0) {
            bit_count += 1;
        }
    }
    return bit_count;
}

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

    var diagnostic_numbers = std.ArrayList(u16).init(alloc);
    defer diagnostic_numbers.deinit();

    const example_numbers = [_][]const u8{ "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001", "00010", "01010" };

    var num_bits: u4 = 0;

    var buf: [16]u8 = undefined;
    while (try input.reader().readUntilDelimiterOrEof(&buf, '\n')) |line| {
        //for (example_numbers) |line| {
        num_bits = @intCast(u4, line.len);
        try diagnostic_numbers.append(try std.fmt.parseInt(u16, line, 2));
    }

    var gamma_rate: u32 = 0;
    var epsilon_rate: u32 = 0;

    {
        var i: u4 = 0;
        while (i < num_bits) : (i += 1) {
            if (get_bitcount(diagnostic_numbers, i) < diagnostic_numbers.items.len / 2) {
                epsilon_rate += std.math.pow(u32, 2, i);
            } else {
                gamma_rate += std.math.pow(u32, 2, i);
            }
        }
    }

    try stdout.print("Gamma & Epsilon rates: {},{} ({})\n", .{ gamma_rate, epsilon_rate, gamma_rate * epsilon_rate });

    var oxygen_numbers = std.ArrayList(u16).init(alloc);
    defer oxygen_numbers.deinit();
    try oxygen_numbers.appendSlice(diagnostic_numbers.items);

    {
        var bn: u4 = num_bits;
        while (bn > 0) : (bn -= 1) {
            const bit_count = get_bitcount(oxygen_numbers, bn - 1);
            const more_ones = bit_count >= oxygen_numbers.items.len - bit_count;
            var i: usize = 0;
            while (i < oxygen_numbers.items.len and oxygen_numbers.items.len > 1) {
                const has_bit: bool = (oxygen_numbers.items[i] & (@as(u16, 1) << bn - 1)) != 0;
                if (has_bit == more_ones) {
                    i += 1;
                } else {
                    _ = oxygen_numbers.orderedRemove(i);
                }
            }
        }
    }
    const oxygen_number: u32 = oxygen_numbers.items[0];

    // Mm, duplication
    var co2_numbers = std.ArrayList(u16).init(alloc);
    defer co2_numbers.deinit();
    try co2_numbers.appendSlice(diagnostic_numbers.items);

    {
        var bn: u4 = num_bits;
        while (bn > 0) : (bn -= 1) {
            const bit_count = get_bitcount(co2_numbers, bn - 1);
            const more_ones = bit_count >= co2_numbers.items.len - bit_count;
            var i: usize = 0;
            while (i < co2_numbers.items.len and co2_numbers.items.len > 1) {
                const has_bit: bool = (co2_numbers.items[i] & (@as(u16, 1) << bn - 1)) != 0;
                if (has_bit == more_ones) {
                    _ = co2_numbers.orderedRemove(i);
                } else {
                    i += 1;
                }
            }
        }
    }
    const co2_number: u32 = co2_numbers.items[0];

    try stdout.print("Oxygen & CO2 numbers: {},{} ({})\n", .{ oxygen_number, co2_number, oxygen_number * co2_number });
}

test "get_bitcount" {
    const example_numbers = [_][]const u8{ "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001", "00010", "01010" };
    const a = std.testing.allocator;
    var numbers = std.ArrayList(u16).init(a);
    defer numbers.deinit();
    for (example_numbers) |line| {
        try numbers.append(try std.fmt.parseInt(u16, line, 2));
    }
    const expected_results = [_]u16{ 7, 5, 8, 7, 5 };
    var i: usize = 0;
    while (i < expected_results.len) : (i += 1) {
        try std.testing.expect(get_bitcount(numbers, @intCast(u4, i)) == expected_results[i]);
    }
}
