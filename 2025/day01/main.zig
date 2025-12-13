const std = @import("std");

fn part1() !void {
    const allocator = std.heap.page_allocator;

    var file = try std.fs.cwd().openFile("input.txt", .{});
    defer file.close();

    const contents = try file.readToEndAlloc(allocator, 1024 * 1024);
    defer allocator.free(contents);

    var dial: i32 = 50;
    var zero_count: u32 = 0;
    var turns = std.mem.splitScalar(u8, contents, '\n');

    while (turns.next()) |turn| {
        if (turn.len == 0) continue;

        const direction: u8 = turn[0];
        // std.log.debug("Turn: {s}", .{turn});
        const distance = try std.fmt.parseInt(i32, turn[1..], 10);

        dial = switch (direction) {
            'L' => (dial - distance),
            'R' => (dial + distance),
            else => {
                continue;
            },
        };

        dial = @mod(dial, 100);

        if (dial == 0) {
            zero_count += 1;
        }
    }

    var stdout_buffer: [256]u8 = undefined;
    var stdout_writer = std.fs.File.stdout().writer(&stdout_buffer);
    const stdout = &stdout_writer.interface;
    try stdout.print("{d}\n", .{zero_count});
    try stdout.flush();
}

fn part2() !void {
    const allocator = std.heap.page_allocator;
    var file = try std.fs.cwd().openFile("input.txt", .{});
    defer file.close();
    const contents = try file.readToEndAlloc(allocator, 1024 * 1024);
    defer allocator.free(contents);

    var dial: i32 = 50;
    var zero_count: u64 = 0;

    var turns = std.mem.splitScalar(u8, contents, '\n');
    while (turns.next()) |turn| {
        if (turn.len == 0) continue;
        const direction: u8 = turn[0];
        const distance: i32 = try std.fmt.parseInt(i32, turn[1..], 10);
        const start: i32 = dial;

        switch (direction) {
            'R' => {
                const end_pos = start + distance;
                const crosses = @divFloor(end_pos, 100) - @divFloor(start, 100);
                zero_count += @intCast(crosses);
                dial = @mod(end_pos, 100);
            },
            'L' => {
                const end_pos = start - distance;
                const crosses = @divFloor(start - 1, 100) - @divFloor(end_pos - 1, 100);
                zero_count += @intCast(crosses);
                dial = @mod(end_pos, 100);
            },
            else => continue,
        }
    }

    var stdout_buffer: [256]u8 = undefined;
    var stdout_writer = std.fs.File.stdout().writer(&stdout_buffer);
    const stdout = &stdout_writer.interface;
    try stdout.print("{d}\n", .{zero_count});
    try stdout.flush();
}

pub fn main() !void {
    try part1();
    try part2();
}
