const std = @import("std");

fn isInvalidId(n: u64) bool {
    var buf: [32]u8 = undefined;
    const s = std.fmt.bufPrint(&buf, "{d}", .{n}) catch return false;

    if (s.len == 0 or (s.len % 2) != 0) return false;

    const half = s.len / 2;
    return std.mem.eql(u8, s[0..half], s[half..]);
}

fn isNewInvalidId(n: u64) bool {
    var buf: [32]u8 = undefined;
    const s = std.fmt.bufPrint(&buf, "{d}", .{n}) catch return false;
    const len = s.len;
    var sub_len: usize = 1;
    if (len == 0) return false;
    while (sub_len <= len / 2) : (sub_len += 1) {
        if (len % sub_len != 0) continue;

        const sub = s[0..sub_len];
        var is_invalid = true;
        var i: usize = sub_len;
        while (i < len) {
            if (!std.mem.eql(u8, sub, s[i .. i + sub_len])) {
                is_invalid = false;
                break;
            }
            i += sub_len;
        }
        if (is_invalid) return true;
    }
    return false;
}

fn sumInvalidInLine(line: []const u8, predicate: fn (u64) bool) !u128 {
    var sum: u128 = 0;

    var parts = std.mem.splitScalar(u8, line, ',');
    while (parts.next()) |raw_part| {
        const part = std.mem.trim(u8, raw_part, " \t\r\n");
        if (part.len == 0) continue;

        const dash_index = std.mem.indexOfScalar(u8, part, '-') orelse
            return error.InvalidRangeFormat;

        const start_str = std.mem.trim(u8, part[0..dash_index], " \t\r\n");
        const end_str = std.mem.trim(u8, part[dash_index + 1 ..], " \t\r\n");

        const start = try std.fmt.parseInt(u64, start_str, 10);
        const end = try std.fmt.parseInt(u64, end_str, 10);
        if (end < start) return error.InvalidRangeOrder;

        var i: u64 = start;
        while (true) {
            if (predicate(i)) sum += @as(u128, i);
            if (i == end) break;
            i += 1;
        }
    }

    return sum;
}

fn part1() !void {
    const allocator = std.heap.page_allocator;

    var file = try std.fs.cwd().openFile("input.txt", .{ .mode = .read_only });
    defer file.close();

    var read_buf: [2]u8 = undefined;
    var f_reader: std.fs.File.Reader = file.reader(&read_buf);

    var line = std.Io.Writer.Allocating.init(allocator);
    defer line.deinit();

    var total: u128 = 0;

    while (true) {
        _ = f_reader.interface.streamDelimiter(&line.writer, '\n') catch |err| {
            if (err == error.EndOfStream) break else return err;
        };
        _ = f_reader.interface.toss(1);

        const s = line.written();
        if (s.len > 0) {
            total += try sumInvalidInLine(s, isInvalidId);
        }
        line.clearRetainingCapacity();
    }

    if (line.written().len > 0) {
        total += try sumInvalidInLine(line.written(), isInvalidId);
        line.clearRetainingCapacity();
    }

    std.debug.print("{d}\n", .{total});
}

fn part2() !void {
    const allocator = std.heap.page_allocator;

    var file = try std.fs.cwd().openFile("input.txt", .{ .mode = .read_only });
    defer file.close();

    var read_buf: [2]u8 = undefined;
    var f_reader: std.fs.File.Reader = file.reader(&read_buf);

    var line = std.Io.Writer.Allocating.init(allocator);
    defer line.deinit();

    var total: u128 = 0;

    while (true) {
        _ = f_reader.interface.streamDelimiter(&line.writer, '\n') catch |err| {
            if (err == error.EndOfStream) break else return err;
        };
        _ = f_reader.interface.toss(1);

        const s = line.written();
        if (s.len > 0) {
            total += try sumInvalidInLine(s, isNewInvalidId);
        }
        line.clearRetainingCapacity();
    }

    if (line.written().len > 0) {
        total += try sumInvalidInLine(line.written(), isNewInvalidId);
        line.clearRetainingCapacity();
    }

    std.debug.print("{d}\n", .{total});
}

pub fn main() !void {
    try part1();
    try part2();
}
