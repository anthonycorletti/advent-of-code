const std = @import("std");

inline fn charAt(line: []const u8, col: usize) u8 {
    return if (col < line.len) line[col] else ' ';
}

inline fn isDigit(ch: u8) bool {
    return ch >= '0' and ch <= '9';
}

fn readVerticalNumberInColumn(
    lines: []const []const u8,
    h: usize,
    c: usize,
) ?u128 {
    var r: usize = 0;
    while (r + 1 < h) : (r += 1) {
        const ch = charAt(lines[r], c);
        if (!isDigit(ch)) continue;

        var n: u128 = 0;
        var rr: usize = r;
        while (rr + 1 < h) : (rr += 1) {
            const d = charAt(lines[rr], c);
            if (!isDigit(d)) break;
            n = n * 10 + @as(u128, d - '0');
        }
        return n;
    }
    return null;
}

fn part1() !void {
    const allocator = std.heap.page_allocator;

    var file = try std.fs.cwd().openFile("input.txt", .{ .mode = .read_only });
    defer file.close();

    const contents = try file.readToEndAlloc(allocator, 1024 * 1024 * 128);
    defer allocator.free(contents);

    var lines: std.ArrayList([]const u8) = .{};
    defer lines.deinit(allocator);

    var it = std.mem.splitScalar(u8, contents, '\n');
    while (it.next()) |raw| {
        const line = std.mem.trimRight(u8, raw, "\r");
        if (line.len == 0) continue;
        try lines.append(allocator, line);
    }
    if (lines.items.len < 2) return error.NotEnoughLines;

    const h: usize = lines.items.len;
    const op_row: []const u8 = lines.items[h - 1];

    var w: usize = 0;
    for (lines.items) |ln| w = @max(w, ln.len);
    if (w == 0) {
        std.debug.print("0\n", .{});
        return;
    }

    var is_blank_col = try allocator.alloc(bool, w);
    defer allocator.free(is_blank_col);

    var col: usize = 0;
    while (col < w) : (col += 1) {
        var blank = true;
        for (lines.items) |ln| {
            if (charAt(ln, col) != ' ') {
                blank = false;
                break;
            }
        }
        is_blank_col[col] = blank;
    }

    var grand_total: u128 = 0;

    var in_span = false;
    var span_start: usize = 0;

    col = 0;
    while (col <= w) : (col += 1) {
        const at_end = (col == w);
        const blank_here = if (at_end) true else is_blank_col[col];

        if (!in_span and !blank_here) {
            in_span = true;
            span_start = col;
            continue;
        }

        if (in_span and blank_here) {
            const span_end_inclusive: usize = col - 1;
            in_span = false;

            var op: u8 = 0;
            var c: usize = span_start;
            while (c <= span_end_inclusive) : (c += 1) {
                const ch = charAt(op_row, c);
                if (ch != ' ') {
                    op = ch;
                    break;
                }
            }
            if (op != '+' and op != '*') return error.InvalidOperator;

            var value: u128 = if (op == '+') 0 else 1;
            var seen_any = false;

            var r: usize = 0;
            while (r + 1 < h) : (r += 1) {
                const ln = lines.items[r];
                if (span_start >= ln.len) continue;

                const end_excl = @min(span_end_inclusive + 1, ln.len);
                const slice = ln[span_start..end_excl];
                const trimmed = std.mem.trim(u8, slice, " \t\r");
                if (trimmed.len == 0) continue;

                const n = try std.fmt.parseInt(u128, trimmed, 10);
                seen_any = true;
                if (op == '+') value += n else value *= n;
            }

            if (seen_any) grand_total += value;
        }
    }

    std.debug.print("{d}\n", .{grand_total});
}

fn part2() !void {
    const allocator = std.heap.page_allocator;

    var file = try std.fs.cwd().openFile("input.txt", .{ .mode = .read_only });
    defer file.close();

    const contents = try file.readToEndAlloc(allocator, 1024 * 1024 * 128);
    defer allocator.free(contents);

    var lines: std.ArrayList([]const u8) = .{};
    defer lines.deinit(allocator);

    var it = std.mem.splitScalar(u8, contents, '\n');
    while (it.next()) |raw| {
        const line = std.mem.trimRight(u8, raw, "\r");
        if (line.len == 0) continue;
        try lines.append(allocator, line);
    }
    if (lines.items.len < 2) return error.NotEnoughLines;

    const h: usize = lines.items.len;
    const op_row: []const u8 = lines.items[h - 1];

    var w: usize = 0;
    for (lines.items) |ln| w = @max(w, ln.len);
    if (w == 0) {
        std.debug.print("0\n", .{});
        return;
    }

    var is_blank_col = try allocator.alloc(bool, w);
    defer allocator.free(is_blank_col);

    var col: usize = 0;
    while (col < w) : (col += 1) {
        var blank = true;
        for (lines.items) |ln| {
            if (charAt(ln, col) != ' ') {
                blank = false;
                break;
            }
        }
        is_blank_col[col] = blank;
    }

    var grand_total: u128 = 0;

    var in_span = false;
    var span_start: usize = 0;

    col = 0;
    while (col <= w) : (col += 1) {
        const at_end = (col == w);
        const blank_here = if (at_end) true else is_blank_col[col];

        if (!in_span and !blank_here) {
            in_span = true;
            span_start = col;
            continue;
        }

        if (in_span and blank_here) {
            const span_end_inclusive: usize = col - 1;
            in_span = false;

            var op: u8 = 0;
            var c0: usize = span_start;
            while (c0 <= span_end_inclusive) : (c0 += 1) {
                const ch = charAt(op_row, c0);
                if (ch != ' ') {
                    op = ch;
                    break;
                }
            }
            if (op != '+' and op != '*') return error.InvalidOperator;

            var value: u128 = if (op == '+') 0 else 1;
            var seen_any = false;

            var c: isize = @as(isize, @intCast(span_end_inclusive));
            const c_min: isize = @as(isize, @intCast(span_start));
            while (c >= c_min) : (c -= 1) {
                const uc: usize = @as(usize, @intCast(c));
                if (readVerticalNumberInColumn(lines.items, h, uc)) |n| {
                    seen_any = true;
                    if (op == '+') value += n else value *= n;
                }
            }

            if (seen_any) grand_total += value;
        }
    }

    std.debug.print("{d}\n", .{grand_total});
}

pub fn main() !void {
    try part1();
    try part2();
}
