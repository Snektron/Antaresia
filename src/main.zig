const std = @import("std");

pub fn main() !void {
    const stdout = try std.io.getStdOut();
    try stdout.write("Oof\n");
}