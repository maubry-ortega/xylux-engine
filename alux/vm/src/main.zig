const std = @import("std");
const vm_lib = @import("vm_lib");

pub fn main() void {
    std.debug.print("AluxVM stub\n", .{});
    vm_lib.hello();
}
