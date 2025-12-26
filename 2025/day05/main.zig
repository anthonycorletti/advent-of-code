const std = @import("std");

const Range = struct {
    hi: u64,
    lo: u64,
};

fn parseRange(line: []const u8) !Range {
    const dash = std.mem.indexOfScalar(u8, line, '-') orelse return error.InvalidRange;
    const lo_str = std.mem.trim(u8, line[0..dash], " \t\r");
    const hi_str = std.mem.trim(u8, line[dash + 1 ..], " \t\r");
    const lo = try std.fmt.parseInt(u64, lo_str, 10);
    const hi = try std.fmt.parseInt(u64, hi_str, 10);
    if (hi < lo) return error.InvalidRangeOrder;
    return .{ .lo = lo, .hi = hi };
}

fn lessThan(_: void, a: Range, b: Range) bool {
    if (a.lo != b.lo) return a.lo < b.lo;
    return a.hi < b.hi;
}

fn mergeRanges(allocator: std.mem.Allocator, ranges: []const Range) !std.ArrayList(Range) {
    var merged: std.ArrayList(Range) = .{};

    for (ranges) |r| {
        if (merged.items.len == 0) {
            try merged.append(allocator, r);
            continue;
        }

        var last = &merged.items[merged.items.len - 1];
        if (r.lo <= last.hi + 1) {
            if (r.hi > last.hi) last.hi = r.hi;
        } else {
            try merged.append(allocator, r);
        }
    }

    return merged;
}

fn isFresh(id: u64, ranges: []const Range) bool {
    var lo: usize = 0;
    var hi: usize = ranges.len;

    while (lo < hi) {
        const mid: usize = lo + (hi - lo) / 2;
        const r = ranges[mid];

        if (id < r.lo) {
            hi = mid;
        } else if (id > r.hi) {
            lo = mid + 1;
        } else {
            return true;
        }
    }
    return false;
}

fn part1() !void {
    const allocator = std.heap.page_allocator;

    var file = try std.fs.cwd().openFile("input.txt", .{ .mode = .read_only });
    defer file.close();

    const contents = try file.readToEndAlloc(allocator, 1024 * 1024 * 64);
    defer allocator.free(contents);

    var ranges: std.ArrayList(Range) = .{};
    defer ranges.deinit(allocator);

    var ids: std.ArrayList(u64) = .{};
    defer ids.deinit(allocator);

    var in_ranges = true;

    var it = std.mem.splitScalar(u8, contents, '\n');
    while (it.next()) |raw_line| {
        const line0 = std.mem.trimRight(u8, raw_line, "\r");
        const line = std.mem.trim(u8, line0, " \t\r");
        if (line.len == 0) {
            in_ranges = false;
            continue;
        }

        if (in_ranges) {
            try ranges.append(allocator, try parseRange(line));
        } else {
            try ids.append(allocator, try std.fmt.parseInt(u64, line, 10));
        }
    }

    std.sort.heap(Range, ranges.items, {}, lessThan);
    var merged = try mergeRanges(allocator, ranges.items);
    defer merged.deinit(allocator);

    var fresh_count: u64 = 0;
    for (ids.items) |id| {
        if (isFresh(id, merged.items)) fresh_count += 1;
    }

    std.debug.print("{d}\n", .{fresh_count});
}

fn mergeSortedRanges(allocator: std.mem.Allocator, sorted: []const Range) !std.ArrayList(Range) {
    var merged: std.ArrayList(Range) = .{};

    for (sorted) |r| {
        if (merged.items.len == 0) {
            try merged.append(allocator, r);
            continue;
        }

        var last = &merged.items[merged.items.len - 1];

        if (r.lo <= last.hi + 1) {
            if (r.hi > last.hi) last.hi = r.hi;
        } else {
            try merged.append(allocator, r);
        }
    }

    return merged;
}

fn part2() !void {
    const allocator = std.heap.page_allocator;

    var file = try std.fs.cwd().openFile("input.txt", .{ .mode = .read_only });
    defer file.close();

    const contents = try file.readToEndAlloc(allocator, 1024 * 1024 * 64);
    defer allocator.free(contents);

    var ranges: std.ArrayList(Range) = .{};
    defer ranges.deinit(allocator);

    //
    var it = std.mem.splitScalar(u8, contents, '\n');
    while (it.next()) |raw_line| {
        const line0 = std.mem.trimRight(u8, raw_line, "\r");
        const line = std.mem.trim(u8, line0, " \t\r");
        if (line.len == 0) break;
        try ranges.append(allocator, try parseRange(line));
    }

    if (ranges.items.len == 0) {
        std.debug.print("0\n", .{});
        return;
    }

    std.sort.heap(Range, ranges.items, {}, lessThan);
    var merged = try mergeSortedRanges(allocator, ranges.items);
    defer merged.deinit(allocator);

    var total_fresh: u128 = 0;
    for (merged.items) |r| {
        total_fresh += (@as(u128, r.hi) - @as(u128, r.lo) + 1);
    }

    std.debug.print("{d}\n", .{total_fresh});
}

pub fn main() !void {
    try part1();
    try part2();
}
