// iterate through each line and make sure it meets both requirements
// return the number of lines that meet both requirements

const std = @import("std");
const log = std.log.scoped(.aoc2024);
const splitScalar = std.mem.splitScalar;

pub fn main() !void {
    const part1_input = std.mem.trimRight(u8, @embedFile("part1.txt"), "\n");
    const result1 = try part1(part1_input);
    log.info("Part 1 result: {}", .{result1});

    const result2 = try part2(part1_input);
    log.info("Part 2 result: {}", .{result2});
}

fn part1(input: []const u8) !u64 {
    var lines = splitScalar(u8, input, '\n');
    var sum: u64 = 0;

    line: while (lines.next()) |line| {
        var levels = std.mem.splitScalar(u8, line, ' ');
        var last: i64 = try std.fmt.parseInt(i64, levels.next().?, 10);
        var last_diff: i64 = 0;
        while (levels.next()) |level| {
            const l = try std.fmt.parseInt(i64, level, 10);

            // Second char or after
            const diff = l - last;

            // Apply rules
            if (diff * last_diff < 0 or @abs(diff) > 3 or @abs(diff) == 0) {
                // Invalid
                continue :line;
            }

            last = l;
            last_diff = diff;
        }
        sum += 1;
    }

    return sum;
}

fn part2(input: []const u8) !usize {
    var lines = std.mem.splitScalar(u8, input, '\n');
    var sum: u64 = 0;
    var levels: [10]i64 = undefined;

    while (lines.next()) |line| {
        var lvls = std.mem.splitScalar(u8, line, ' ');

        // Read everything
        var idx: usize = 0;
        while (lvls.next()) |level| : (idx += 1) {
            const l = try std.fmt.parseInt(i64, level, 10);
            levels[idx] = l;
        }
        const cap = idx;

        // Loop over with skipping 1 level
        skip: for (0..cap) |skip_it| {
            var last: ?i64 = null;
            var last_diff: i64 = 0;

            for (0.., levels[0..cap]) |i, l| {
                if (skip_it == i) {
                    continue;
                }

                if (last == null) {
                    last = l;
                    continue;
                }

                const diff = l - last.?;
                if (diff * last_diff < 0 or @abs(diff) > 3 or @abs(diff) == 0) {
                    continue :skip;
                }

                last = l;
                last_diff = diff;
            }
            sum += 1;
            break;
        }
    }

    return sum;
}
