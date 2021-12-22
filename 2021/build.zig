const std = @import("std");

pub fn build(b: *std.build.Builder) void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const alloc = arena.allocator();

    // Standard target options allows the person running `zig build` to choose
    // what target to build for. Here we do not override the defaults, which
    // means any target is allowed, and the default is native. Other options
    // for restricting supported target set are available.
    const target = b.standardTargetOptions(.{});

    // Standard release options allow the person running `zig build` to select
    // between Debug, ReleaseSafe, ReleaseFast, and ReleaseSmall.
    const mode = b.standardReleaseOptions();

    var day: i32 = 1;
    while (day <= 25) : (day += 1) {
        const file_path = std.fmt.allocPrint(alloc, "src/day{}.zig", .{day}) catch unreachable;
        defer alloc.free(file_path);
        if (std.fs.cwd().access(file_path, std.fs.File.OpenFlags{ .read = true })) {
            const exe_name = std.fmt.allocPrint(alloc, "day{}", .{day}) catch unreachable;
            defer alloc.free(exe_name);

            const exe = b.addExecutable(exe_name, file_path);
            exe.setTarget(target);
            exe.setBuildMode(mode);
            exe.install();

            const run_cmd = exe.run();
            run_cmd.step.dependOn(b.getInstallStep());

            const input_arg = std.fmt.allocPrint(alloc, "inputs/day{}.input", .{day}) catch unreachable;
            defer alloc.free(input_arg);

            if (b.args) |args| {
                run_cmd.addArgs(args);
            } else {
                run_cmd.addArg(input_arg);
            }

            const run_name = std.fmt.allocPrint(alloc, "run{}", .{day}) catch unreachable;
            defer alloc.free(run_name);
            const run_step = b.step(run_name, "Run dayN");
            run_step.dependOn(&run_cmd.step);
        } else |_| continue;
    }
}
