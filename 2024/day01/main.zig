// pair up the smallest number in the left and right lists, second smallest and so on
// find out how far apart the teo nuimbers are and add up all of those distances

const std = @import("std");
const Allocator = std.mem.Allocator;
const GPA = std.heap.GeneralPurposeAllocator;
const ArrayList = std.ArrayList;
const log = std.log.scoped(.aoc2024);
const split = std.mem.split;

pub fn main() !void {
    var gpa = GPA(.{}){};
    defer _ = gpa.deinit();
    const alloc = gpa.allocator();

    const part1_input = @embedFile("part1.txt");
    const result1 = try part1(part1_input, alloc);
    log.info("Part 1 result: {d}", .{result1});
    const result2 = try part2(part1_input, alloc);
    log.info("Part 2 result: {d}", .{result2});
}

fn part1(input: []const u8, alloc: Allocator) !usize {
    var col1 = ArrayList(usize).init(alloc);
    var col2 = ArrayList(usize).init(alloc);

    defer col1.deinit();
    defer col2.deinit();

    var lines = split(u8, input, "\n");
    while (lines.next()) |line| {
        if (line.len == 0) break;
        var iter = std.mem.tokenize(u8, line, " ");
        const a = try std.fmt.parseInt(usize, iter.next().?, 10);
        const b = try std.fmt.parseInt(usize, iter.next().?, 10);
        try col1.append(a);
        try col2.append(b);
    }

    std.sort.heap(usize, col1.items, {}, LessThan(usize).lessThanFn);
    std.sort.heap(usize, col2.items, {}, LessThan(usize).lessThanFn);

    var sum: usize = 0;
    for (col1.items, 0..) |a, i| {
        const b = col2.items[i];
        // log.debug("a: {d}, b: {d}", .{ a, b });
        sum += @max(a, b) - @min(a, b);
    }

    return sum;
}

fn LessThan(T: anytype) type {
    return struct {
        pub fn lessThanFn(_: void, a: T, b: T) bool {
            return a < b;
        }
    };
}

fn part2(input: []const u8, alloc: Allocator) !usize {
    var col1 = ArrayList(usize).init(alloc);
    var col2 = ArrayList(usize).init(alloc);

    defer col1.deinit();
    defer col2.deinit();

    var lines = split(u8, input, "\n");
    while (lines.next()) |line| {
        if (line.len == 0) break;
        var iter = std.mem.tokenize(u8, line, " ");
        const a = try std.fmt.parseInt(usize, iter.next().?, 10);
        const b = try std.fmt.parseInt(usize, iter.next().?, 10);
        try col1.append(a);
        try col2.append(b);
    }

    std.sort.heap(usize, col1.items, {}, LessThan(usize).lessThanFn);
    std.sort.heap(usize, col2.items, {}, LessThan(usize).lessThanFn);

    var sum: usize = 0;
    for (col1.items) |a| {
        const count = std.mem.count(usize, col2.items, &[1]usize{a});
        sum += count * a;
    }

    return sum;
}
