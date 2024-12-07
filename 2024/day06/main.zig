const std = @import("std");
const log = std.log.scoped(.aoc2024);

const trimRight = std.mem.trimRight;
const tokenizeScalar = std.mem.tokenizeScalar;
const copyForwards = std.mem.copyForwards;
const splitScalar = std.mem.splitScalar;

pub fn main() !void {
    const part1_input = trimRight(u8, @embedFile("part1.txt"), "\n");
    const result1 = try part1(part1_input, 130, 130);
    log.info("Part 1 result: {}", .{result1});
    const result2 = try part2(part1_input, 130, 130);
    log.info("Part 2 result: {}", .{result2});
}

const Point = struct {
    col: usize,
    row: usize,

    fn move(self: Point, dir: Direction) Point {
        return switch (dir) {
            .up => Point{ .col = self.col, .row = self.row - 1 },
            .down => Point{ .col = self.col, .row = self.row + 1 },
            .left => Point{ .col = self.col - 1, .row = self.row },
            .right => Point{ .col = self.col + 1, .row = self.row },
        };
    }
};

const Direction = enum(u8) {
    up = 0b1,
    down = 0b10,
    left = 0b100,
    right = 0b1000,

    pub fn turn(self: *Direction) void {
        switch (self.*) {
            .up => self.* = .right,
            .right => self.* = .down,
            .down => self.* = .left,
            .left => self.* = .up,
        }
    }
};

fn part1(input: []const u8, comptime rows: usize, comptime cols: usize) !u64 {
    var grid: [rows][cols]u8 = undefined;
    var iter = tokenizeScalar(u8, input, '\n');
    var i: usize = 0;
    var pos: Point = undefined;
    var dir: Direction = .up;

    while (iter.next()) |line| : (i += 1) {
        copyForwards(u8, &grid[i], line);
        var j: usize = 0;
        for (line) |char| {
            if (char == '^') {
                pos = Point{ .col = j, .row = i };
            }
            j += 1;
        }
    }

    while (pos.col > 0 and pos.col < cols - 1 and pos.row < rows - 1 and pos.row > 0) {
        const tmp = pos.move(dir);
        if (grid[tmp.row][tmp.col] == '#') {
            dir.turn();
        } else {
            grid[tmp.row][tmp.col] = 'X';
            pos = tmp;
        }
    }

    var sum: u64 = 0;
    for (grid) |row| {
        for (row) |cell| {
            if (cell == 'X') {
                sum += 1;
            }
        }
    }

    return sum;
}

fn part2(input: []const u8, comptime rows: usize, comptime cols: usize) !u64 {
    var sum: u64 = 0;

    var start_grid: [rows][cols]u8 = undefined;
    var iter = tokenizeScalar(u8, input, '\n');
    var i: usize = 0;
    var start_pos: Point = undefined;
    const start_dir: Direction = .up;

    while (iter.next()) |line| : (i += 1) {
        var j: usize = 0;
        for (line) |char| {
            if (char == '^') {
                start_pos = Point{ .col = j, .row = i };
            }
            if (char == '#') {
                start_grid[i][j] = '#';
            } else {
                start_grid[i][j] = 0;
            }
            j += 1;
        }
    }

    i = 0;
    while (i < rows) : (i += 1) {
        var j: usize = 0;
        outer: while (j < cols) : (j += 1) {
            var grid = start_grid;
            grid[i][j] = '#';
            var pos = start_pos;
            var dir = start_dir;
            while (pos.col > 0 and pos.col < cols - 1 and pos.row < rows - 1 and pos.row > 0) {
                const tmp = pos.move(dir);
                if (grid[tmp.row][tmp.col] == '#') {
                    dir.turn();
                } else {
                    if (grid[tmp.row][tmp.col] & @intFromEnum(dir) > 0) {
                        sum += 1;
                        continue :outer;
                    }
                    grid[tmp.row][tmp.col] |= @intFromEnum(dir);
                    pos = tmp;
                }
            }
        }
    }

    return sum;
}
