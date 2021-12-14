const std = @import("std");

const Pair = struct {
    a: u8,
    b: u8,
};

fn apply_insertion_rules(alloc: *std.mem.Allocator, rules: anytype, input: std.ArrayList(u8)) !std.ArrayList(u8) {
    var output = std.ArrayList(u8).init(alloc);
    var i: usize = 0;
    while (i < input.items.len - 1) : (i += 1) {
        try output.append(input.items[i]);
        try output.append(rules.get(Pair{ .a = input.items[i], .b = input.items[i + 1] }).?);
    }
    try output.append(input.items[input.items.len - 1]);
    return output;
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

    var insertion_rules = std.AutoArrayHashMap(Pair, u8).init(alloc);
    defer insertion_rules.deinit();

    var polymer_template = std.ArrayList(u8).init(alloc);
    defer polymer_template.deinit();

    var parse_map = false;
    var buf: [300]u8 = undefined;
    while (try input.reader().readUntilDelimiterOrEof(&buf, '\n')) |line| {
        if (line.len == 0) {
            parse_map = true;
            continue;
        }

        if (parse_map) {
            var it = std.mem.split(line, " -> ");

            var lr = it.next().?;
            try insertion_rules.put(Pair{ .a = lr[0], .b = lr[1] }, it.next().?[0]);
        } else {
            try polymer_template.appendSlice(line);
        }
    }

    var step: u32 = 0;
    while (step < 10) : (step += 1) {
        var new_polymer_template = try apply_insertion_rules(alloc, insertion_rules, polymer_template);
        polymer_template.deinit();
        polymer_template = new_polymer_template;
    }

    var element_quantities = std.AutoHashMap(u8, u32).init(alloc);
    defer element_quantities.deinit();
    for (polymer_template.items) |e| {
        var gop = try element_quantities.getOrPut(e);
        if (!gop.found_existing) {
            gop.value_ptr.* = 0;
        }
        gop.value_ptr.* += 1;
    }

    var min_element: u8 = undefined;
    var min_element_count: u32 = std.math.maxInt(u32);
    var max_element: u8 = undefined;
    var max_element_count: u32 = 0;

    var elem_it = element_quantities.iterator();
    while (elem_it.next()) |val| {
        if (val.value_ptr.* > max_element_count) {
            max_element = val.key_ptr.*;
            max_element_count = val.value_ptr.*;
        }
        if (val.value_ptr.* < min_element_count) {
            min_element = val.key_ptr.*;
            min_element_count = val.value_ptr.*;
        }
    }
    std.debug.print("Elements: {c} ({}), {c} ({}) = {}\n", .{ min_element, min_element_count, max_element, max_element_count, max_element_count - min_element_count });
}
