const std = @import("std");
const path = std.os.path;
const Builder = std.build.Builder;

pub fn build(b: *Builder) void {
    const exe = b.addExecutable("antaresia", "src/main.zig");
    const run = b.step("run", "Run the program");
    const run_cmd = b.addCommand(".", b.env_map, [][]const u8{exe.getOutputPath()});
    run.dependOn(&run_cmd.step);
    run_cmd.step.dependOn(&exe.step);
}