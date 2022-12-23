fn range_contains_range(range1: &str, range2: &str) -> bool {
    let (r1, r2) = range1.split_once("-").unwrap();
    let (r3, r4) = range2.split_once("-").unwrap();

    let r1 = r1.parse::<i32>().unwrap();
    let r2 = r2.parse::<i32>().unwrap();
    let r3 = r3.parse::<i32>().unwrap();
    let r4 = r4.parse::<i32>().unwrap();

    r1 <= r3 && r2 >= r4
}

fn range_overlaps_range(range1: &str, range2: &str) -> bool {
    let (r1, r2) = range1.split_once("-").unwrap();
    let (r3, r4) = range2.split_once("-").unwrap();

    let r1 = r1.parse::<i32>().unwrap();
    let r2 = r2.parse::<i32>().unwrap();
    let r3 = r3.parse::<i32>().unwrap();
    let r4 = r4.parse::<i32>().unwrap();

    r1 <= r3 && r2 >= r3 || r1 <= r4 && r2 >= r4
}

fn main() {
    let mut input = include_str!("../input.txt").to_string();
    input.pop(); // remove trailing newline
    let mut result_p1 = 0;
    let mut result_p2 = 0;

    for line in input.lines() {
        let (r1, r2) = line.split_once(",").unwrap();

        // if r1 contains r2 or if r2 contains r1, increment result_p1
        if range_contains_range(r1, r2) || range_contains_range(r2, r1) {
            result_p1 += 1;
        }

        // if r1 and r2 overlap, increment result_p2
        if range_overlaps_range(r1, r2) || range_overlaps_range(r2, r1) {
            result_p2 += 1;
        }
    }

    println!("Part 1: {}", result_p1);
    println!("Part 2: {}", result_p2);
}
