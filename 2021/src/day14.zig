const std = @import("std");

const Pair = struct {
    a: u8,
    b: u8,
};

const ScoreObj = struct {
    min: u8,
    min_count: u64,
    max: u8,
    max_count: u64,
    score: u64,
};

const Polymer = std.AutoArrayHashMap(Pair, u64);

fn apply_insertion_rules(alloc: std.mem.Allocator, rules: anytype, input: Polymer) !Polymer {
    var output = Polymer.init(alloc);

    var it = input.iterator();
    while (it.next()) |e| {
        const match = rules.get(e.key_ptr.*).?;
        const left_pair = Pair{ .a = e.key_ptr.a, .b = match };
        const right_pair = Pair{ .a = match, .b = e.key_ptr.b };

        const left_gop = try output.getOrPutValue(left_pair, 0);
        left_gop.value_ptr.* += e.value_ptr.*;
        const right_gop = try output.getOrPutValue(right_pair, 0);
        right_gop.value_ptr.* += e.value_ptr.*;
    }

    return output;
}

fn get_polymer_score(alloc: std.mem.Allocator, poly: Polymer, last_letter: u8) !ScoreObj {
    var element_quantities = std.AutoHashMap(u8, u64).init(alloc);
    defer element_quantities.deinit();
    // Can't count the last letter, and we can't know which one it was in the array of pairs, so add it here to make up for it
    try element_quantities.put(last_letter, 1);
    var polymer_it = poly.iterator();
    while (polymer_it.next()) |e| {
        // Only counts one half of the pair, to avoid double counting. Last letter is accounted for above
        const gop1 = try element_quantities.getOrPutValue(e.key_ptr.a, 0);
        gop1.value_ptr.* += e.value_ptr.*;
    }

    var min_element: u8 = undefined;
    var min_element_count: u64 = std.math.maxInt(u64);
    var max_element: u8 = undefined;
    var max_element_count: u64 = 0;

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

    // Only need the score, but makes printing nicer
    return ScoreObj{
        .min = min_element,
        .min_count = min_element_count,
        .max = max_element,
        .max_count = max_element_count,
        .score = max_element_count - min_element_count,
    };
}

pub fn main() anyerror!void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const alloc = arena.allocator();
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

    var polymer_pairs = Polymer.init(alloc);
    defer polymer_pairs.deinit();
    var last_letter: u8 = undefined;

    var parse_map = false;
    var buf: [300]u8 = undefined;
    while (try input.reader().readUntilDelimiterOrEof(&buf, '\n')) |line| {
        if (line.len == 0) {
            parse_map = true;
            continue;
        }

        if (parse_map) {
            var it = std.mem.split(u8, line, " -> ");

            var lr = it.next().?;
            try insertion_rules.put(Pair{ .a = lr[0], .b = lr[1] }, it.next().?[0]);
        } else {
            var i: usize = 0;
            while (i < line.len - 1) : (i += 1) {
                const p = Pair{ .a = line[i], .b = line[i + 1] };
                const gop = try polymer_pairs.getOrPutValue(p, 0);
                gop.value_ptr.* += 1;
            }
            last_letter = line[line.len - 1];
        }
    }

    var p1_score: ScoreObj = undefined;
    var step: u32 = 0;
    while (step < 40) : (step += 1) {
        if (step == 10) {
            p1_score = try get_polymer_score(alloc, polymer_pairs, last_letter);
        }

        var new_polymer_pairs = try apply_insertion_rules(alloc, insertion_rules, polymer_pairs);
        polymer_pairs.deinit();
        polymer_pairs = new_polymer_pairs;
    }
    const p2_score = try get_polymer_score(alloc, polymer_pairs, last_letter);

    try stdout.print("Polymer score after 10 steps: {c} ({}), {c} ({}) = {}\n", p1_score);
    try stdout.print("Polymer score after 40 steps: {c} ({}), {c} ({}) = {}\n", p2_score);
}
