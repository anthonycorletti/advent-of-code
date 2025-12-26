const std = @import("std");

fn pow10(exponent: usize) u128 {
    var result: u128 = 1;
    var i: usize = 0;
    while (i < exponent) : (i += 1) {
        result *= 10;
    }
    return result;
}

fn findLargestJoltageFrom(line: []const u8, start_idx: usize, digits_left: usize) ?u128 {
    if (digits_left == 0) return 0;
    if (start_idx >= line.len) return null;

    var digit: i32 = 9;
    while (digit >= 0) : (digit -= 1) {
        const digit_u8: u8 = @intCast(digit);
        const digit_char: u8 = '0' + digit_u8;

        if (std.mem.indexOfScalar(u8, line[start_idx..], digit_char)) |relative_idx| {
            const next_start = start_idx + relative_idx + 1;
            if (digits_left == 1) {
                return @as(u128, digit_u8);
            }
            if (findLargestJoltageFrom(line, next_start, digits_left - 1)) |tail_value| {
                const magnitude = pow10(digits_left - 1);
                return (@as(u128, digit_u8) * magnitude) + tail_value;
            }
        }
    }

    return null;
}

fn findLargestJoltage(line: []const u8, opts: struct { digits: usize = 2 }) !u128 {
    const digit_count = opts.digits;
    if (digit_count == 0) return error.InvalidDigitCount;

    if (findLargestJoltageFrom(line, 0, digit_count)) |value| {
        return value;
    }

    return error.NoNumberWithLength;
}

fn part2() !void {
    const allocator = std.heap.page_allocator;
    var file = try std.fs.cwd().openFile("input.txt", .{ .mode = .read_only });
    defer file.close();

    var read_buf: [2]u8 = undefined;
    var file_reader: std.fs.File.Reader = file.reader(&read_buf);

    var line = std.io.Writer.Allocating.init(allocator);
    defer line.deinit();

    var total: u128 = 0;

    while (true) {
        _ = file_reader.interface.streamDelimiter(&line.writer, '\n') catch |err| {
            if (err == error.EndOfStream) break else return err;
        };
        _ = file_reader.interface.toss(1);

        const s = line.written();
        if (s.len > 0) {
            total += try findLargestJoltage(s, .{ .digits = 12 });
        }
        line.clearRetainingCapacity();
    }

    if (line.written().len > 0) {
        total += try findLargestJoltage(line.written(), .{ .digits = 12 });
        line.clearRetainingCapacity();
    }

    std.debug.print("{d}\n", .{total});
}

fn part1() !void {
    const allocator = std.heap.page_allocator;
    var file = try std.fs.cwd().openFile("input.txt", .{ .mode = .read_only });
    defer file.close();

    var read_buf: [2]u8 = undefined;
    var file_reader: std.fs.File.Reader = file.reader(&read_buf);

    var line = std.io.Writer.Allocating.init(allocator);
    defer line.deinit();

    var total: u128 = 0;

    while (true) {
        _ = file_reader.interface.streamDelimiter(&line.writer, '\n') catch |err| {
            if (err == error.EndOfStream) break else return err;
        };
        _ = file_reader.interface.toss(1);

        const s = line.written();
        if (s.len > 0) {
            total += try findLargestJoltage(s, .{});
        }
        line.clearRetainingCapacity();
    }

    if (line.written().len > 0) {
        total += try findLargestJoltage(line.written(), .{});
        line.clearRetainingCapacity();
    }

    std.debug.print("{d}\n", .{total});
}

pub fn main() !void {
    try part1();
    try part2();
}
