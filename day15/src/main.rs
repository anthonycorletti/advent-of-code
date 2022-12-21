use itertools::Itertools;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Point(isize, isize);

impl Point {
    // manhattan distance
    fn distance_to(&self, other: &Self) -> usize {
        let Point(x1, y1) = self;
        let Point(x2, y2) = other;
        x1.abs_diff(*x2) + y1.abs_diff(*y2)
    }

    fn tuning_frequency(&self) -> u64 {
        (4_000_000 * self.0 as u64) + self.1 as u64
    }
}

impl From<(i32, i32)> for Point {
    fn from(value: (i32, i32)) -> Self {
        let (x, y) = value;
        Point(x as isize, y as isize)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Sensor {
    location: Point,
    beacon: Point,
    range: usize,
}

impl Sensor {
    fn new(location: Point, beacon: Point) -> Self {
        let range = location.distance_to(&beacon);
        Sensor {
            location,
            beacon,
            range,
        }
    }
}

impl From<(Point, Point)> for Sensor {
    fn from(value: (Point, Point)) -> Self {
        let (location, beacon) = value;
        Sensor::new(location, beacon)
    }
}

mod parser {
    use super::*;
    use anyhow::{anyhow, Result};
    use nom::{
        bytes::complete::take_till,
        character::{
            complete::{i32, newline},
            is_alphabetic, is_digit,
        },
        combinator::{map, recognize},
        multi::separated_list0,
        sequence::{pair, preceded},
        Finish, IResult,
    };

    fn till_number(s: &str) -> IResult<&str, &str> {
        take_till(|c: char| c.is_ascii_digit() || c == '-')(s)
    }

    fn prefixed_number(s: &str) -> IResult<&str, i32> {
        preceded(till_number, i32)(s)
    }

    fn point(s: &str) -> IResult<&str, Point> {
        map(pair(prefixed_number, prefixed_number), Point::from)(s)
    }

    fn sensor(s: &str) -> IResult<&str, Sensor> {
        map(pair(point, point), Sensor::from)(s)
    }

    fn sensors(s: &str) -> IResult<&str, Vec<Sensor>> {
        separated_list0(newline, sensor)(s)
    }

    pub(crate) fn parse(s: &str) -> Result<Vec<Sensor>> {
        let (_, result) = sensors(s).finish().map_err(|e| anyhow!("{e}"))?;
        Ok(result)
    }
}

#[derive(Debug)]
struct RowRange(isize, isize);

impl RowRange {
    fn overlaps(&self, other: &Self) -> bool {
        other.1 >= self.0 && self.1 >= other.0
    }

    fn merge(&mut self, other: &Self) {
        *self = RowRange(self.0.min(other.0), self.1.max(other.1));
    }

    fn count_positions(&self) -> usize {
        self.0.abs_diff(self.1) + 1
    }
}

impl Sensor {
    fn can_detect(&self, point: &Point) -> bool {
        self.location.distance_to(point) <= self.range
    }

    fn row_range_sensed(&self, row: isize) -> Option<RowRange> {
        let distance_to_row = self.location.1.abs_diff(row);
        if distance_to_row > self.range {
            return None;
        }

        // The spread indicates how much of the Manhattan distance for detection
        // is remaining to 'spread' out to the left and right. Essentially half
        // the width of the detection zone on this row.
        let spread = self.range - distance_to_row;
        let range_start = self.location.0.saturating_sub_unsigned(spread);
        let range_end = self.location.0.saturating_add_unsigned(spread);
        Some(RowRange(range_start, range_end))
    }

    fn beacon_on_row(&self, row: isize) -> Option<Point> {
        if self.beacon.1 == row {
            return Some(self.beacon);
        }
        None
    }

    fn gap_size(&self, other: &Self) -> Option<usize> {
        let distance = self.location.distance_to(&other.location);
        let total_range = self.range + other.range;
        if total_range >= distance {
            return None;
        }
        Some(distance - total_range - 1)
    }

    /// Calculate the formula for the line that lies in the gap between two
    /// Sensor detection ranges. The line will lie diagonally just outside
    /// the range of `self`.
    fn diagonal_between(&self, other: &Self) -> Diagonal {
        let Point(x1, y1) = self.location;
        let Point(x2, y2) = other.location;
        let offset = self.range + 1;

        // Here, we identify two points on the diagonal line. We'll pick points just
        // outside the cardinal direction points of the `self` sensor range.
        let (p1x, p1y) = if x2 > x1 {
            (x1.saturating_add_unsigned(offset), y1)
        } else {
            (x1.saturating_sub_unsigned(offset), y1)
        };
        let (p2x, p2y) = if y2 > y1 {
            (x1, y1.saturating_add_unsigned(offset))
        } else {
            (x1, y1.saturating_sub_unsigned(offset))
        };

        // We know that the slope will either be 1 or -1, since these lines
        // are diagonals.
        let slope = (p2x - p1x) / (p2y - p1y);
        let intercept = p1y - (slope * p1x);
        if slope > 0 {
            Diagonal::Positive(intercept)
        } else {
            Diagonal::Negative(intercept)
        }
    }
}

#[derive(Debug)]
enum Diagonal {
    Positive(isize),
    Negative(isize),
}

impl Diagonal {
    /// Identify the point where two Diagonal lines intersect.
    fn intersect(&self, other: &Self) -> Option<Point> {
        // It's simple geometry! Which explains why it was so hard for me
        // to implement. Uses the formula for the two lines to calculate the
        // intersecting point, with some shortcuts because we know the slope
        // will either be positive or negative one for both lines, and if
        // the lines have the same slope, they're parallel and we can bail.
        use Diagonal::*;
        let (neg, pos) = match (self, other) {
            (Positive(pos), Negative(neg)) => (neg, pos),
            (Negative(neg), Positive(pos)) => (neg, pos),
            (Positive(_), Positive(_)) => return None,
            (Negative(_), Negative(_)) => return None,
        };
        let x = (neg - pos) / 2;
        let y = x + pos;
        Some(Point(x, y))
    }
}

fn solve_p1() -> u32 {
    let row = 2_000_000;

    let input = include_str!("../input.txt");
    let mut sensors = parser::parse(input).unwrap();
    sensors.sort_unstable();

    let mut ranges: Vec<RowRange> = Vec::new();
    for range in sensors.iter().flat_map(|s| s.row_range_sensed(row)) {
        // manhattan distance
        if let Some(last_rng) = ranges.last_mut() {
            if last_rng.overlaps(&range) {
                last_rng.merge(&range);
            }
            continue;
        }
        ranges.push(range);
    }

    // manhattan distance
    let sensed_on_row = ranges.iter().map(|r| r.count_positions()).sum::<usize>();

    // manhattan distance
    let beacons_on_row = sensors
        .iter()
        .filter_map(|s| s.beacon_on_row(row))
        .unique()
        .count();

    let definitely_not_beacons = sensed_on_row - beacons_on_row;

    return definitely_not_beacons as u32;
}

fn solve_p2() -> u64 {
    let input = include_str!("../input.txt");
    let mut sensors = parser::parse(input).unwrap();
    sensors.sort_unstable();

    let mut diagonal_gaps = Vec::new();
    for (sensor1, sensor2) in sensors.iter().tuple_combinations() {
        let Some(gap) = sensor1.gap_size(sensor2) else { continue; };
        if gap == 1 {
            diagonal_gaps.push(sensor1.diagonal_between(sensor2));
        }
    }

    // Identify all the points where these one-wide gaps intersect.
    let intersects = diagonal_gaps
        .iter()
        .tuple_combinations()
        .flat_map(|(diag1, diag2)| diag1.intersect(diag2))
        .unique()
        .collect_vec();

    'outer: for intersect in intersects {
        for sensor in sensors.iter() {
            if sensor.can_detect(&intersect) {
                continue 'outer;
            }
        }
        return intersect.tuning_frequency().into();
    }

    // Freak out if we can't find an intersection that can't be detected.
    panic!("Could not find the beacon!");
}

fn main() {
    println!("Part 1: {}", solve_p1());
    println!("Part 2: {}", solve_p2());
}
