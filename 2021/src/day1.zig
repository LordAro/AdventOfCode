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
    const contents = file.reader().readAllAlloc(alloc, 10_000_000);

    std.log.info("File contents: {s}", .{contents});
}
