const std = @import("std");

const Packet = struct {
    version: u3,
    typeid: u3,
    length: u32,
    value: u64,
    subpackets: std.ArrayList(Packet),
};

fn input_to_bits(alloc: std.mem.Allocator, input: []const u8) !std.ArrayList(bool) {
    var program_bits = std.ArrayList(bool).init(alloc);
    for (input) |c| {
        const digit: u4 = @as(u4, @intCast(try std.fmt.charToDigit(c, 16)));
        try program_bits.append(((digit >> 3) & 1) != 0);
        try program_bits.append(((digit >> 2) & 1) != 0);
        try program_bits.append(((digit >> 1) & 1) != 0);
        try program_bits.append(((digit >> 0) & 1) != 0);
    }
    return program_bits;
}

fn bits_to_num(blob: []bool) u32 {
    var out: u32 = 0;
    for (blob, 0..) |b, i| {
        out |= @as(u32, @intFromBool(b)) << @as(u5, @intCast(blob.len)) - @as(u5, @intCast(i)) - 1;
    }
    return out;
}

fn parse_packet(alloc: std.mem.Allocator, program: []bool) Packet {
    const version = @as(u3, @intCast(bits_to_num(program[0..3])));
    const typeid = @as(u3, @intCast(bits_to_num(program[3..6])));

    var packet = Packet{ .version = version, .typeid = typeid, .length = 0, .value = 0, .subpackets = undefined };
    var pc: u32 = 6;
    if (typeid == 4) {
        // literal
        var val: u64 = 0; // max 8 groups
        var n: u1 = 1;
        while (n != 0) {
            n = @as(u1, @intCast(bits_to_num(program[pc .. pc + 1])));
            pc += 1;
            const segment = bits_to_num(program[pc .. pc + 4]);
            val = (val << 4) + segment;
            pc += 4;
        }
        packet.length = pc;
        packet.value = val;
        packet.subpackets = std.ArrayList(Packet).init(alloc);
    } else {
        // operator
        const lengthtypeid = @as(u1, @intCast(bits_to_num(program[pc .. pc + 1])));
        pc += 1;

        if (lengthtypeid == 0) {
            const subpacketlength = bits_to_num(program[pc .. pc + 15]);
            pc += 15;

            packet.length = pc + subpacketlength;
            packet.subpackets = parse_packets(alloc, program[pc .. pc + subpacketlength]);
        } else {
            const num_subpackets = bits_to_num(program[pc .. pc + 11]);
            pc += 11;

            packet.subpackets = std.ArrayList(Packet).init(alloc);
            var i: u32 = 0;
            while (i < num_subpackets) : (i += 1) {
                const sp = parse_packet(alloc, program[pc..]);
                packet.subpackets.append(sp) catch unreachable;
                pc += sp.length;
            }
            packet.length = pc;
        }

        switch (packet.typeid) {
            0 => { // sum
                packet.value = 0;
                for (packet.subpackets.items) |sp| {
                    packet.value += sp.value;
                }
            },
            1 => { // product
                packet.value = 1;
                for (packet.subpackets.items) |sp| {
                    packet.value *= sp.value;
                }
            },
            2 => { // minimum
                packet.value = std.math.maxInt(u32);
                for (packet.subpackets.items) |sp| {
                    packet.value = @min(packet.value, sp.value);
                }
            },
            3 => { // maximum
                packet.value = 0;
                for (packet.subpackets.items) |sp| {
                    packet.value = @max(packet.value, sp.value);
                }
            },
            4 => unreachable, // literals
            5 => { // greater than
                packet.value = if (packet.subpackets.items[0].value > packet.subpackets.items[1].value) 1 else 0;
            },
            6 => { // less than
                packet.value = if (packet.subpackets.items[0].value < packet.subpackets.items[1].value) 1 else 0;
            },
            7 => { // less than
                packet.value = if (packet.subpackets.items[0].value == packet.subpackets.items[1].value) 1 else 0;
            },
        }
    }
    return packet;
}

fn get_packet_version(p: Packet) u64 {
    var version: u64 = 0;
    for (p.subpackets.items) |sp| {
        version += get_packet_version(sp);
    }
    version += p.version;
    return version;
}

fn parse_packets(alloc: std.mem.Allocator, program: []bool) std.ArrayList(Packet) {
    var packets = std.ArrayList(Packet).init(alloc);

    var pc: usize = 0;
    while (pc < program.len - 8) {
        const p = parse_packet(alloc, program[pc..]);
        packets.append(p) catch unreachable;
        pc += p.length;
    }
    return packets;
}

fn packetlist_deinit(pl: std.ArrayList(Packet)) void {
    for (pl.items) |p| {
        packetlist_deinit(p.subpackets);
    }
    pl.deinit();
}

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
        std.process.exit(1);
    };
    defer input.close();

    var buf: [4096]u8 = undefined;
    var program_bits = try input_to_bits(alloc, (try input.reader().readUntilDelimiterOrEof(&buf, '\n')).?);
    defer program_bits.deinit();

    const packets = parse_packets(alloc, program_bits.items);
    defer packetlist_deinit(packets);

    try stdout.print("Packet version sum: {}\n", .{get_packet_version(packets.items[0])});
    try stdout.print("Packet value: {}\n", .{packets.items[0].value});
}

test "example1" {
    const test_alloc = std.testing.allocator;
    const program = try input_to_bits(test_alloc, "D2FE28");
    defer program.deinit();
    std.debug.print("\n", .{});

    const packets = parse_packets(test_alloc, program.items);
    defer packetlist_deinit(packets);

    try std.testing.expect(packets.items.len == 1);
    const p = packets.items[0];
    try std.testing.expectEqual(p.version, 6);
    try std.testing.expectEqual(p.typeid, 4);
    try std.testing.expectEqual(p.length, 21);
    try std.testing.expectEqual(p.value, 2021);
}

test "example2" {
    const test_alloc = std.testing.allocator;
    const program = try input_to_bits(test_alloc, "38006F45291200");
    defer program.deinit();
    std.debug.print("\n", .{});

    const packets = parse_packets(test_alloc, program.items);
    defer packetlist_deinit(packets);

    try std.testing.expectEqual(packets.items.len, 1);
    const p = packets.items[0];
    try std.testing.expectEqual(p.subpackets.items.len, 2);
    try std.testing.expectEqual(p.subpackets.items[0].value, 10);
    try std.testing.expectEqual(p.subpackets.items[1].value, 20);
}

test "example3" {
    const test_alloc = std.testing.allocator;
    const program = try input_to_bits(test_alloc, "EE00D40C823060");
    defer program.deinit();
    std.debug.print("\n", .{});

    const packets = parse_packets(test_alloc, program.items);
    defer packetlist_deinit(packets);

    try std.testing.expectEqual(packets.items.len, 1);
    const p = packets.items[0];
    try std.testing.expectEqual(p.subpackets.items.len, 3);
}

test "example4" {
    const test_alloc = std.testing.allocator;
    const program = try input_to_bits(test_alloc, "8A004A801A8002F478");
    defer program.deinit();
    std.debug.print("\n", .{});

    const packets = parse_packets(test_alloc, program.items);
    defer packetlist_deinit(packets);

    try std.testing.expectEqual(packets.items.len, 1);
    const p = packets.items[0];
    try std.testing.expectEqual(p.version, 4);
    try std.testing.expectEqual(p.subpackets.items.len, 1);
}

test "p2_examples" {
    const test_alloc = std.testing.allocator;
    const inputs = [_][]const u8{
        "C200B40A82",
        "04005AC33890",
        "880086C3E88112",
        "CE00C43D881120",
        "D8005AC2A8F0",
        "F600BC2D8F",
        "9C005AC2F8F0",
        "9C0141080250320F1802104A08",
    };

    const expected_value = [_]u32{
        3, 54, 7, 9, 1, 0, 0, 1,
    };

    for (inputs, expected_value) |input, expected| {
        const program = try input_to_bits(test_alloc, input);
        defer program.deinit();
        std.debug.print("\n", .{});

        const packet = parse_packet(test_alloc, program.items);
        defer packetlist_deinit(packet.subpackets);

        try std.testing.expectEqual(packet.value, expected);
    }
}
