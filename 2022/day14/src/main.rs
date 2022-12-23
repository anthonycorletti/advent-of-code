use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::ops::Add;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Point(u32, u32);

#[derive(Debug, Default, Clone, Copy)]
struct Offset(i32, i32);

impl Point {
    fn offset_from(&self, other: &Self) -> Offset {
        let Point(x1, y1) = self;
        let Point(x2, y2) = other;
        match (x1.cmp(x2), y1.cmp(y2)) {
            (Ordering::Less, Ordering::Less) => Offset(-1, -1),
            (Ordering::Less, Ordering::Equal) => Offset(-1, 0),
            (Ordering::Less, Ordering::Greater) => Offset(-1, 1),
            (Ordering::Equal, Ordering::Less) => Offset(0, -1),
            (Ordering::Equal, Ordering::Equal) => Offset(0, 0),
            (Ordering::Equal, Ordering::Greater) => Offset(0, 1),
            (Ordering::Greater, Ordering::Less) => Offset(1, -1),
            (Ordering::Greater, Ordering::Equal) => Offset(1, 0),
            (Ordering::Greater, Ordering::Greater) => Offset(1, 1),
        }
    }
}

impl Add<Offset> for Point {
    type Output = Point;

    fn add(self, rhs: Offset) -> Self::Output {
        let Point(px, py) = self;
        let Offset(ox, oy) = rhs;
        let x = px.saturating_add_signed(ox);
        let y = py.saturating_add_signed(oy);
        Point(x, y)
    }
}

mod parser {
    use super::*;
    use anyhow::{anyhow, Result};
    use nom::{
        bytes::complete::tag,
        character::complete::{newline, u32},
        multi::separated_list1,
        sequence::separated_pair,
        Finish, IResult,
    };

    fn point(s: &str) -> IResult<&str, Point> {
        let (s, (first, second)) = separated_pair(u32, tag(","), u32)(s)?;
        Ok((s, Point(first, second)))
    }

    fn point_list(s: &str) -> IResult<&str, Vec<Point>> {
        separated_list1(tag(" -> "), point)(s)
    }

    fn point_lists(s: &str) -> IResult<&str, Vec<Vec<Point>>> {
        separated_list1(newline, point_list)(s)
    }

    pub(crate) fn parse(s: &str) -> Result<Vec<Vec<Point>>> {
        let (_, result) = point_lists(s).finish().map_err(|e| anyhow!("{e}"))?;
        Ok(result)
    }
}

struct RockLineIter {
    start: Point,        // The point where the rock line starts
    end: Point,          // The point where the rock line ends
    offset: Offset,      // The incremental change from `start` to `end`
    next: Option<Point>, // The next item to return from this iterator
}

trait RockLine {
    fn rock_line(self) -> RockLineIter;
}

impl RockLine for (Point, Point) {
    fn rock_line(self) -> RockLineIter {
        let (start, end) = self;
        let offset = end.offset_from(&start);
        RockLineIter {
            start,
            end,
            offset,
            next: Some(start), // The first point returned is the start
        }
    }
}

impl Iterator for RockLineIter {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next {
            None => None, // This is how we know when `RockLineIter` is empty
            Some(current) => {
                self.next = if current == self.end {
                    None
                } else {
                    Some(current + self.offset)
                };

                Some(current)
            }
        }
    }
}

#[derive(Debug)]
enum GrainStatus {
    MovedTo(Point),
    StoppedAt(Point),
    LostToTheAbyss,
}

#[derive(Debug, Clone)]
struct CaveMap {
    obstacles: HashSet<Point>,
    entrypoint: Point,
    depth: u32,
}

impl CaveMap {
    fn new(obstacles: HashSet<Point>) -> Self {
        let depth = obstacles
            .iter()
            .map(|point| point.1)
            .max()
            .unwrap_or_default();

        let entrypoint = Point(500, 0);

        CaveMap {
            obstacles,
            entrypoint,
            depth,
        }
    }

    fn add_sand(&mut self) -> GrainStatus {
        let mut sand = self.entrypoint;

        loop {
            let sand_flow = self.try_move_sand(sand);

            match sand_flow {
                GrainStatus::MovedTo(point) => sand = point,

                GrainStatus::StoppedAt(point) => {
                    self.obstacles.insert(point);
                    break sand_flow;
                }

                GrainStatus::LostToTheAbyss => break sand_flow,
            }
        }
    }

    fn try_move_sand(&self, sand: Point) -> GrainStatus {
        let offsets = [Offset(0, 1), Offset(-1, 1), Offset(1, 1)];

        for offset in offsets {
            let try_pos = sand + offset;

            if self.obstacles.contains(&try_pos) {
                continue;
            }

            if sand.1 >= self.depth {
                return GrainStatus::LostToTheAbyss;
            }

            return GrainStatus::MovedTo(try_pos);
        }

        GrainStatus::StoppedAt(sand)
    }
}

#[derive(Debug, Clone)]
pub struct FillMap {
    obstacles: HashSet<Point>,
    entrypoint: Point,
    depth: u32,
}

impl FillMap {
    /// Unpack a `CaveMap` into a `FillMap`
    fn from(grid_map: CaveMap) -> Self {
        // Get the attributes from the `CaveMap`
        let CaveMap {
            obstacles,
            entrypoint,
            depth,
        } = grid_map;

        // Adjust the depth to represent the floor. Hey, look, there's that grain of
        // sand we thought was gone forever, breathing a huge sigh of relief. Good
        // for him!
        let depth = depth + 2;

        // Now it's a `FillMap`!
        FillMap {
            obstacles,
            entrypoint,
            depth,
        }
    }

    /// From a given Point, return an array indicating which points a grain of sand
    /// can flow into (e.g., that aren't blocked by an obstacle or the floor).
    fn get_neighbors(&self, point: Point) -> [Option<Point>; 3] {
        // The same three potential moves as the first part
        let offsets = [Offset(0, 1), Offset(-1, 1), Offset(1, 1)];

        // Array to hold the neighbors that can be moved to
        let mut neighbors = [None; 3];

        // For each possible offset...
        for (idx, offset) in offsets.iter().enumerate() {
            // The position we might move to.
            let try_pos = point + *offset;

            // If there's an obstacle there, skip it. Can't move there.
            if self.obstacles.contains(&try_pos) {
                continue;
            }

            // If there's floor there, skip it. Can't move there.
            if try_pos.1 >= self.depth {
                continue;
            }

            // Otherwise, we can move there. Add this point to our neighbors array.
            neighbors[idx] = Some(try_pos);
        }

        // Return the list of neighbors
        neighbors
    }

    /// Calculate the number of sand grains it'll take to fill in the pile and
    /// block off the entrypoint. Using Dijkstra's Algorithm! Nah, just kidding,
    /// it's a breadth-first search.
    fn sand_capacity(&self) -> u32 {
        let mut queue = VecDeque::from([self.entrypoint]);
        let mut visited = HashSet::new();
        let mut counted = 0; // Keep up with the number of grains

        // So long as we've got positions to try moving _from_...
        while let Some(point) = queue.pop_back() {
            // If we've visited this space before, skip it. Been here, done that.
            if visited.contains(&point) {
                continue;
            }
            visited.insert(point); // Mark `point` as visited
            counted += 1; // Count this grain of sand

            // For each reachable neighbor point from the current point
            for neighbor in self.get_neighbors(point).iter().flatten() {
                // If we've visited that point before, skip it.
                if visited.contains(neighbor) {
                    continue;
                }

                // Add that point to the list of points to visit
                queue.push_front(*neighbor);
            }
        }

        counted // Return the number of grains of sand we counted
    }
}

fn solve_p1() -> i32 {
    let input = include_str!("../input.txt");
    let point_lists = parser::parse(input).unwrap();
    let mut obstacles = HashSet::new();
    for point_list in point_lists {
        for point_pair in point_list.into_iter().tuple_windows::<(_, _)>() {
            for rock_point in point_pair.rock_line() {
                obstacles.insert(rock_point);
            }
        }
    }

    let mut cave_map = CaveMap::new(obstacles.clone());
    for grains in 1..10_000 {
        // When we find the first grain of sand that falls into the infinite
        // abyss, we stop and return the current grain count minus one as
        // the number of grains _before_ this poor soul was lost to the void.
        if let GrainStatus::LostToTheAbyss = cave_map.add_sand() {
            return grains - 1;
        }
    }

    unreachable!();
}

fn solve_p2() -> u32 {
    let input = include_str!("../input.txt");
    let point_lists = parser::parse(input).unwrap();
    let mut obstacles = HashSet::new();
    for point_list in point_lists {
        for point_pair in point_list.into_iter().tuple_windows::<(_, _)>() {
            for rock_point in point_pair.rock_line() {
                obstacles.insert(rock_point);
            }
        }
    }

    let mut cave_map = CaveMap::new(obstacles.clone());
    let mut fill_map = FillMap::from(cave_map);

    return fill_map.sand_capacity();
}

fn main() {
    println!("Part 1: {}", solve_p1());
    println!("Part 2: {}", solve_p2());
}
