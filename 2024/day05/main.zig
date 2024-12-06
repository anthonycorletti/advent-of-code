const std = @import("std");

const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const GPA = std.heap.GeneralPurposeAllocator;
const log = std.log.scoped(.aoc2024);

const eql = std.mem.eql;
const parseInt = std.fmt.parseInt;
const sort = std.mem.sort;
const splitScalar = std.mem.splitScalar;
const tokenizeScalar = std.mem.tokenizeScalar;

const Rule = struct {
    before: u16,
    after: u16,
};

pub fn main() !void {
    var gpa = GPA(.{}){};
    defer _ = gpa.deinit();

    const alloc = gpa.allocator();
    const part1_input = std.mem.trimRight(u8, @embedFile("part1.txt"), "\n");
    const result1 = try part1(part1_input, alloc, false);
    log.info("Part 1 result: {}", .{result1});
    const result2 = try part1(part1_input, alloc, true);
    log.info("Part 2 result: {}", .{result2});
}

fn part1(input: []const u8, alloc: Allocator, does_reorder: bool) !u32 {
    var rules = ArrayList(Rule).init(alloc);
    defer rules.deinit();
    var lines = splitScalar(u8, input, '\n');
    var readingRules = true;
    var sum: u32 = 0;
    while (lines.next()) |line| {
        // log.info("{s}", .{line});
        if (line.len == 0) {
            readingRules = false;
            continue;
        }
        if (readingRules) {
            // por = page order rule
            var por = tokenizeScalar(u8, line, '|');
            const rule = Rule{
                .before = try parseInt(u16, por.next().?, 10),
                .after = try parseInt(u16, por.next().?, 10),
            };
            try rules.append(rule);
        } else {
            var pages = ArrayList(u16).init(alloc);
            defer pages.deinit();
            var po = tokenizeScalar(u8, line, ',');
            while (po.next()) |page| {
                try pages.append(try parseInt(u16, page, 10));
            }
            var tmp = try pages.clone();
            defer tmp.deinit();
            sort(u16, pages.items, rules.items, lessByRules);
            if (does_reorder) {
                if (!eql(u16, tmp.items, pages.items)) {
                    sum += pages.items[pages.items.len / 2];
                }
            } else {
                if (eql(u16, tmp.items, pages.items)) {
                    sum += pages.items[pages.items.len / 2];
                }
            }
        }
    }
    return sum;
}

fn lessByRules(rules: []Rule, a: u16, b: u16) bool {
    for (rules) |r| {
        if (a == r.before and b == r.after) return true;
        if (a == r.after and b == r.before) return false;
    }
    return false;
}
