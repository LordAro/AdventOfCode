const std = @import("std");

fn get_bits(blob: []bool) u32 {
    var out: u32 = 0;
    for (blob) |b, i| {
        out |= @as(u32, @boolToInt(b)) << @intCast(u5, blob.len) - @intCast(u5, i) - 1;
    }
    return out;
}

const Packet = struct {
    version: u3,
    typeid: u3,
    length: u32,
    value: u32,
    subpackets: std.ArrayList(Packet),
};

fn packetlist_deinit(pl: std.ArrayList(Packet)) void {
    for (pl.items) |p| {
        packetlist_deinit(p.subpackets);
    }
    pl.deinit();
}

fn get_packet_version(p: Packet) u32 {
    var version: u32 = 0;
    for (p.subpackets.items) |sp| {
        version += get_packet_version(sp);
    }
    version += p.version;
    return version;
}

fn parse_packet(alloc: *std.mem.Allocator, program: []bool) Packet {
    const version = @intCast(u3, get_bits(program[0..3]));
    const typeid = @intCast(u3, get_bits(program[3..6]));

    var packet = Packet{ .version = version, .typeid = typeid, .length = 0, .value = 0, .subpackets = undefined };
    var pc: u32 = 6;
    if (typeid == 4) {
        // literal
        var val: u32 = 0; // max 4 groups
        var n: u1 = 1;
        while (n != 0) {
            n = @intCast(u1, get_bits(program[pc .. pc + 1]));
            pc += 1;
            const segment = get_bits(program[pc .. pc + 4]);
            val = (val << 4) + segment;
            pc += 4;
        }
        packet.length = pc;
        packet.value = val;
        packet.subpackets = std.ArrayList(Packet).init(alloc);
    } else {
        // operator
        const lengthtypeid = @intCast(u1, get_bits(program[pc .. pc + 1]));
        pc += 1;

        if (lengthtypeid == 0) {
            const subpacketlength = get_bits(program[pc .. pc + 15]);
            pc += 15;

            packet.length = pc + subpacketlength;
            packet.subpackets = parse_packets(alloc, program[pc .. pc + subpacketlength]);
        } else {
            const num_subpackets = get_bits(program[pc .. pc + 11]);
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
    }
    return packet;
}

fn parse_packets(alloc: *std.mem.Allocator, program: []bool) std.ArrayList(Packet) {
    var packets = std.ArrayList(Packet).init(alloc);

    var pc: usize = 0;
    while (pc < program.len - 8) {
        const p = parse_packet(alloc, program[pc..]);
        packets.append(p) catch unreachable;
        pc += p.length;
    }
    return packets;
}

fn input_to_bits(alloc: *std.mem.Allocator, input: []const u8) !std.ArrayList(bool) {
    var program_bits = std.ArrayList(bool).init(alloc);
    for (input) |c| {
        const digit: u4 = @intCast(u4, try std.fmt.charToDigit(c, 16));
        try program_bits.append(((digit >> 3) & 1) != 0);
        try program_bits.append(((digit >> 2) & 1) != 0);
        try program_bits.append(((digit >> 1) & 1) != 0);
        try program_bits.append(((digit >> 0) & 1) != 0);
    }
    return program_bits;
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

    var buf: [4096]u8 = undefined;
    var program_bits = try input_to_bits(alloc, (try input.reader().readUntilDelimiterOrEof(&buf, '\n')).?);
    defer program_bits.deinit();

    var packets = parse_packets(alloc, program_bits.items);
    defer packetlist_deinit(packets);

    std.debug.print("{any}\n", .{packets.items});
    var version_sum: u32 = 0;
    for (packets.items) |p| {
        version_sum += get_packet_version(p);
    }
    try stdout.print("Packet version sum: {}\n", .{version_sum});
}

test "example1" {
    var test_alloc = std.testing.allocator;
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
    var test_alloc = std.testing.allocator;
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
    var test_alloc = std.testing.allocator;
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
    var test_alloc = std.testing.allocator;
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
