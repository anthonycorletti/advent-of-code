const std = @import("std");
const log = std.log.scoped(.aoc2024);
const splitScalar = std.mem.splitScalar;

pub fn main() !void {
    const part1_input = std.mem.trimRight(u8, @embedFile("part1.txt"), "\n");
    const result1 = try part1(part1_input);
    log.info("Part 1 result: {}", .{result1});
    const result2 = try part2(part1_input);
    log.info("Part 1 result: {}", .{result2});
}

fn part1(input: []const u8) !u64 {
    // loop through the input which is a string of characters, and find the number of times you find a "mul(number,number)"
    // each time you find one, multiply the two numbers and add it to the sum
    // return the sum

    var sum: u64 = 0;
    var i: usize = 0;

    while (i < input.len - 4) {
        if (!std.mem.eql(u8, input[i .. i + 4], "mul(")) {
            i += 1;
            continue;
        }
        i += 4;

        const first_num = scanNumber(input[i..]);
        if (first_num.len == 0) continue;
        i += first_num.len;

        if (input[i] != ',') continue;
        i += 1;

        const second_num = scanNumber(input[i..]);
        if (second_num.len == 0) continue;
        i += second_num.len;

        if (input[i] != ')') continue;
        i += 1;

        const a = try std.fmt.parseInt(u32, first_num, 10);
        const b = try std.fmt.parseInt(u32, second_num, 10);
        sum += a * b;
    }

    return sum;
}

fn part2(input: []const u8) !u64 {
    var sum: u64 = 0;
    var i: usize = 0;
    while (i < input.len) {
        if (match(input[i..], "don't()")) {
            while (!match(input[i..], "do()") and i < input.len) {
                i += 1;
            }
        }

        if (!match(input[i..], "mul(")) {
            i += 1;
            continue;
        }
        i += 4;

        const first_num = scanNumber(input[i..]);
        if (first_num.len == 0) continue;
        i += first_num.len;

        if (input[i] != ',') continue;
        i += 1;

        const second_num = scanNumber(input[i..]);
        if (second_num.len == 0) continue;
        i += second_num.len;

        if (input[i] != ')') continue;
        i += 1;

        const a = try std.fmt.parseInt(u32, first_num, 10);
        const b = try std.fmt.parseInt(u32, second_num, 10);
        sum += a * b;
    }

    return sum;
}

fn scanNumber(buf: []const u8) []const u8 {
    var len: usize = 0;
    for (buf) |c| {
        if (std.ascii.isAlphanumeric(c)) {
            len += 1;
        } else {
            break;
        }
    }
    return buf[0..len];
}

fn match(buf: []const u8, s: []const u8) bool {
    if (s.len > buf.len) return false;
    return std.mem.eql(u8, buf[0..s.len], s);
}
