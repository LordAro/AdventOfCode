const std = @import("std");

const Coord = struct {
    x: i32,
    y: i32,
};

const Image = std.AutoHashMap(Coord, bool);

fn get_num(img: Image, centre: Coord, missing_is_lit: bool) u9 {
    var res: u9 = 0;
    var y = centre.y - 1;
    while (y <= centre.y + 1) : (y += 1) {
        var x = centre.x - 1;
        while (x <= centre.x + 1) : (x += 1) {
            res = res << 1;
            if (img.getPtr(Coord{ .x = x, .y = y })) |v| {
                if (v.*) {
                    res |= 1;
                }
            } else if (missing_is_lit) {
                res |= 1;
            }
        }
    }
    return res;
}

const BoundingBox = struct {
    tl: Coord,
    br: Coord,
};

fn get_minmax(img: Image) BoundingBox {
    var tl = Coord{ .x = 0, .y = 0 };
    var br = Coord{ .x = 0, .y = 0 };
    var it = img.keyIterator();
    while (it.next()) |c| {
        if (c.x < tl.x) tl.x = c.x;
        if (c.y < tl.y) tl.y = c.y; // y is negative
        if (c.x > br.x) br.x = c.x;
        if (c.y > br.y) br.y = c.y;
    }
    return .{ .tl = tl, .br = br };
}

fn count_lit(img: Image) usize {
    var lit_pixels: usize = 0;
    var it = img.valueIterator();
    while (it.next()) |p| {
        if (p.*) lit_pixels += 1;
    }
    return lit_pixels;
}

fn enhance(alloc: std.mem.Allocator, input: Image, program: []u8, missing_is_lit: bool) !Image {
    const bb = get_minmax(input);

    var enhanced = Image.init(alloc);

    var y: i32 = bb.tl.y - 1;
    while (y <= bb.br.y + 1) : (y += 1) {
        var x: i32 = bb.tl.x - 1;
        while (x <= bb.br.x + 1) : (x += 1) {
            const pixel = Coord{ .x = x, .y = y };
            const enhanced_pixel = program[get_num(input, pixel, missing_is_lit)] == '#';
            try enhanced.put(pixel, enhanced_pixel);
        }
    }
    return enhanced;
}

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const alloc = arena.allocator();
    const stdout = std.io.getStdOut().writer();

    var args_iter = std.process.args();
    _ = args_iter.skip(); // program name
    const input_file = args_iter.next() orelse unreachable;
    const input = std.fs.cwd().openFile(input_file, .{}) catch |err| {
        std.log.err("Could not open {s} due to: {}", .{ input_file, err });
        std.os.exit(1);
    };
    defer input.close();

    var buf: [1024]u8 = undefined;
    var input_reader = input.reader();

    const enhancement = try alloc.dupe(u8, (try input_reader.readUntilDelimiterOrEof(&buf, '\n')).?);
    defer alloc.free(enhancement);
    std.debug.assert(enhancement.len == 512);

    _ = try input_reader.readUntilDelimiterOrEof(&buf, '\n'); // blank line

    var grid = Image.init(alloc);

    var row_n: usize = 0;
    while (try input_reader.readUntilDelimiterOrEof(&buf, '\n')) |line| : (row_n += 1) {
        for (line) |c, col_n| {
            try grid.put(Coord{ .x = @intCast(i32, col_n), .y = @intCast(i32, row_n) }, c == '#');
        }
    }

    var num_lit2: usize = 0;
    var num_lit50: usize = 0;

    var previous_grid = grid;

    var enhance_step: usize = 0;
    while (enhance_step < 50) : (enhance_step += 1) {
        if (enhance_step == 2) {
            num_lit2 = count_lit(previous_grid);
        }
        var enhanced = try enhance(alloc, previous_grid, enhancement, enhance_step % 2 == 1 and enhancement[0] == '#');
        previous_grid.deinit();
        previous_grid = enhanced;
    }
    num_lit50 = count_lit(previous_grid);
    try stdout.print("Number of pixels after 2 enhancements: {}\n", .{num_lit2});
    try stdout.print("Number of pixels after 50 enhancements: {}\n", .{num_lit50});
}

test "get num" {
    var alloc = std.testing.allocator;
    var img = Image.init(alloc);
    defer img.deinit();

    //# . . # .
    //#[. . .].
    //#[# . .]#
    //.[. # .].
    //. . # # #
    try img.put(Coord{ .x = 0, .y = 0 }, true);
    try img.put(Coord{ .x = 1, .y = 0 }, false);
    try img.put(Coord{ .x = 2, .y = 0 }, false);
    try img.put(Coord{ .x = 3, .y = 0 }, true);
    try img.put(Coord{ .x = 4, .y = 0 }, false);
    try img.put(Coord{ .x = 0, .y = 1 }, true);
    try img.put(Coord{ .x = 1, .y = 1 }, false);
    try img.put(Coord{ .x = 2, .y = 1 }, false);
    try img.put(Coord{ .x = 3, .y = 1 }, false);
    try img.put(Coord{ .x = 4, .y = 1 }, false);
    try img.put(Coord{ .x = 0, .y = 2 }, true);
    try img.put(Coord{ .x = 1, .y = 2 }, true);
    try img.put(Coord{ .x = 2, .y = 2 }, false);
    try img.put(Coord{ .x = 3, .y = 2 }, false);
    try img.put(Coord{ .x = 4, .y = 2 }, true);
    try img.put(Coord{ .x = 0, .y = 3 }, false);
    try img.put(Coord{ .x = 1, .y = 3 }, false);
    try img.put(Coord{ .x = 2, .y = 3 }, true);
    try img.put(Coord{ .x = 3, .y = 3 }, false);
    try img.put(Coord{ .x = 4, .y = 3 }, false);
    try img.put(Coord{ .x = 0, .y = 4 }, false);
    try img.put(Coord{ .x = 1, .y = 4 }, false);
    try img.put(Coord{ .x = 2, .y = 4 }, true);
    try img.put(Coord{ .x = 3, .y = 4 }, true);
    try img.put(Coord{ .x = 4, .y = 4 }, true);

    try std.testing.expectEqual(get_num(img, Coord{ .x = 2, .y = 2 }, false), 34);
}
