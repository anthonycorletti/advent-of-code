const std = @import("std");

fn part1() !void {
    const allocator = std.heap.page_allocator;

    var file = try std.fs.cwd().openFile("input.txt", .{ .mode = .read_only });
    defer file.close();

    const contents = try file.readToEndAlloc(allocator, 1024 * 1024 * 32);
    defer allocator.free(contents);

    var rows: std.ArrayList([]const u8) = .{};
    defer rows.deinit(allocator);

    var it = std.mem.splitScalar(u8, contents, '\n');
    while (it.next()) |raw_line| {
        const line = std.mem.trimRight(u8, raw_line, "\r");
        if (line.len == 0) continue;
        try rows.append(allocator, line);
    }

    if (rows.items.len == 0) {
        std.debug.print("0\n", .{});
        return;
    }

    const h: usize = rows.items.len;
    const w: usize = rows.items[0].len;
    var accessible: u64 = 0;

    var y: usize = 0;
    while (y < h) : (y += 1) {
        var x: usize = 0;
        while (x < w) : (x += 1) {
            if (rows.items[y][x] != '@') continue;

            var neighbors: u32 = 0;

            var dy: i32 = -1;
            while (dy <= 1) : (dy += 1) {
                var dx: i32 = -1;
                while (dx <= 1) : (dx += 1) {
                    if (dx == 0 and dy == 0) continue;

                    const ny_i = @as(i32, @intCast(y)) + dy;
                    const nx_i = @as(i32, @intCast(x)) + dx;

                    if (ny_i < 0 or nx_i < 0) continue;
                    const ny: usize = @intCast(ny_i);
                    const nx: usize = @intCast(nx_i);
                    if (ny >= h or nx >= w) continue;

                    if (rows.items[ny][nx] == '@') neighbors += 1;
                }
            }

            if (neighbors < 4) accessible += 1;
        }
    }

    std.debug.print("{d}\n", .{accessible});
}

fn readGrid(allocator: std.mem.Allocator, contents: []const u8) !struct {
    rows: std.ArrayList([]const u8),
    w: usize,
    h: usize,
} {
    var rows: std.ArrayList([]const u8) = .{};

    var it = std.mem.splitScalar(u8, contents, '\n');
    while (it.next()) |raw_line| {
        const line = std.mem.trimRight(u8, raw_line, "\r");
        if (line.len == 0) continue;
        try rows.append(allocator, line);
    }

    if (rows.items.len == 0) return error.EmptyInput;

    const h: usize = rows.items.len;
    const w: usize = rows.items[0].len;

    for (rows.items) |line| {
        if (line.len != w) return error.NonRectangularGrid;
    }

    return .{ .rows = rows, .w = w, .h = h };
}

inline fn idxOf(w: usize, x: usize, y: usize) usize {
    return y * w + x;
}

fn countNeighborsOccupied(occupied: []const bool, w: usize, h: usize, x: usize, y: usize) u8 {
    var cnt: u8 = 0;

    var dy: i32 = -1;
    while (dy <= 1) : (dy += 1) {
        var dx: i32 = -1;
        while (dx <= 1) : (dx += 1) {
            if (dx == 0 and dy == 0) continue;

            const ny_i: i32 = @as(i32, @intCast(y)) + dy;
            const nx_i: i32 = @as(i32, @intCast(x)) + dx;

            if (ny_i < 0 or nx_i < 0) continue;
            const ny: usize = @intCast(ny_i);
            const nx: usize = @intCast(nx_i);
            if (ny >= h or nx >= w) continue;

            if (occupied[idxOf(w, nx, ny)]) cnt += 1;
        }
    }

    return cnt;
}

fn part2() !void {
    const allocator = std.heap.page_allocator;

    var file = try std.fs.cwd().openFile("input.txt", .{ .mode = .read_only });
    defer file.close();

    const contents = try file.readToEndAlloc(allocator, 1024 * 1024 * 64);
    defer allocator.free(contents);

    var grid = try readGrid(allocator, contents);
    defer grid.rows.deinit(allocator);

    const w = grid.w;
    const h = grid.h;
    const n_cells = w * h;

    var occupied = try allocator.alloc(bool, n_cells);
    defer allocator.free(occupied);
    @memset(occupied, false);

    var degree = try allocator.alloc(u8, n_cells);
    defer allocator.free(degree);
    @memset(degree, 0);

    var total_rolls: u64 = 0;

    for (grid.rows.items, 0..) |line, y| {
        for (line, 0..) |c, x| {
            if (c == '@') {
                occupied[idxOf(w, x, y)] = true;
                total_rolls += 1;
            }
        }
    }

    var y: usize = 0;
    while (y < h) : (y += 1) {
        var x: usize = 0;
        while (x < w) : (x += 1) {
            const id = idxOf(w, x, y);
            if (!occupied[id]) continue;
            degree[id] = countNeighborsOccupied(occupied, w, h, x, y);
        }
    }

    var queue: std.ArrayList(usize) = .{};
    defer queue.deinit(allocator);

    var i: usize = 0;
    while (i < n_cells) : (i += 1) {
        if (occupied[i] and degree[i] < 4) {
            try queue.append(allocator, i);
        }
    }

    var removed: u64 = 0;
    var head: usize = 0;

    while (head < queue.items.len) : (head += 1) {
        const id = queue.items[head];
        if (!occupied[id]) continue;
        if (degree[id] >= 4) continue;

        occupied[id] = false;
        removed += 1;

        const x0: usize = id % w;
        const y0: usize = id / w;

        var dy: i32 = -1;
        while (dy <= 1) : (dy += 1) {
            var dx: i32 = -1;
            while (dx <= 1) : (dx += 1) {
                if (dx == 0 and dy == 0) continue;

                const ny_i: i32 = @as(i32, @intCast(y0)) + dy;
                const nx_i: i32 = @as(i32, @intCast(x0)) + dx;

                if (ny_i < 0 or nx_i < 0) continue;
                const ny: usize = @intCast(ny_i);
                const nx: usize = @intCast(nx_i);
                if (ny >= h or nx >= w) continue;

                const nid = idxOf(w, nx, ny);
                if (!occupied[nid]) continue;

                degree[nid] -= 1;

                if (degree[nid] == 3) {
                    try queue.append(allocator, nid);
                }
            }
        }
    }

    std.debug.print("{d}\n", .{removed});
}

pub fn main() !void {
    try part1();
    try part2();
}
