const std = @import("std");
const log = std.log.scoped(.aoc2024);

pub fn main() !void {
    const part1_input = std.mem.trimRight(u8, @embedFile("part1.txt"), "\n");
    const result1 = try part1(part1_input);
    log.info("Part 1 result: {}", .{result1});
    const result2 = try part2(part1_input);
    log.info("Part 1 result: {}", .{result2});
}

fn part1(input: []const u8) !u64 {
    var sum: u64 = 0;

    var n_cols: i32 = 0;
    for (input, 0..) |v, idx| {
        if (v == '\n') {
            n_cols = @intCast(idx);
            break;
        }
    }

    for (0..input.len) |i| {
        if (input[i] != 'X') continue;
        if (checkXmasValue(input, i, 1)) sum += 1;
        if (checkXmasValue(input, i, -1)) sum += 1;
        if (checkXmasValue(input, i, n_cols + 1)) sum += 1;
        if (checkXmasValue(input, i, -(n_cols + 1))) sum += 1;
        if (checkXmasValue(input, i, n_cols + 2)) sum += 1;
        if (checkXmasValue(input, i, n_cols)) sum += 1;
        if (checkXmasValue(input, i, -(n_cols + 2))) sum += 1;
        if (checkXmasValue(input, i, -(n_cols))) sum += 1;
    }

    return sum;
}

fn part2(input: []const u8) !u64 {
    var sum: u64 = 0;
    var n_cols: i32 = 0;

    for (input, 0..) |v, idx| {
        if (v == '\n') {
            n_cols = @intCast(idx);
            break;
        }
    }
    for (0..input.len) |i| {
        if (input[i] != 'A') continue;
        const top_left = getCorner(i, -(n_cols + 2), input) orelse continue;
        const top_right = getCorner(i, -n_cols, input) orelse continue;
        const bot_left = getCorner(i, n_cols, input) orelse continue;
        const bot_right = getCorner(i, n_cols + 2, input) orelse continue;

        if (((top_left == 'M' and bot_right == 'S') or (top_left == 'S' and bot_right == 'M')) and
            ((top_right == 'M' and bot_left == 'S') or (top_right == 'S' and bot_left == 'M')))
        {
            sum += 1;
        }
    }

    return sum;
}

fn getCorner(base: usize, offset: i32, d: []const u8) ?u8 {
    const signed_base: i64 = @intCast(base);
    const result = signed_base + offset;
    return if (result < 0 or result >= d.len) null else d[@as(usize, @intCast(result))];
}

fn checkXmasValue(input: []const u8, i: usize, delta: i32) bool {
    const idx2: usize = addToUsize(i, delta) orelse return false;
    const idx3: usize = addToUsize(i, 2 * delta) orelse return false;
    const idx4: usize = addToUsize(i, 3 * delta) orelse return false;
    if (idx2 > input.len or idx3 > input.len or idx4 > input.len) return false;
    return (input[idx2] == 'M' and input[idx3] == 'A' and input[idx4] == 'S');
}

fn addToUsize(base: usize, offset: i32) ?usize {
    const signed_base: i64 = @intCast(base);
    const result = signed_base + offset;
    return if (result < 0) null else @as(usize, @intCast(result));
}
