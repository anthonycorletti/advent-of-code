use std::collections::{BTreeMap, BTreeSet};
use std::{cmp, env, fs, process};

const WIDTH: i64 = 7;

const CACHE_LEN: usize = 20;

fn parse_args() -> (String, i64) {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("usage: {} <input_file> <num_rocks>", args[0]);
        process::exit(1);
    }
    let filename: String = args[1].clone();
    let num_rocks: i64;
    if args[2].parse::<i64>().is_err() {
        panic!("parse_args(): expected a number, got: {}", &args[2]);
    } else {
        num_rocks = args[2].parse::<i64>().unwrap();
    }
    return (filename, num_rocks);
}

fn read_input(contents: &str) -> Vec<char> {
    return contents.chars().collect::<Vec<char>>();
}

fn free(settled: &BTreeSet<(i64, i64)>, x: i64, y: i64) -> bool {
    return (x >= 0) && (x < WIDTH) && (y > 0) && !settled.contains(&(x, y));
}

fn can_move(
    settled: &BTreeSet<(i64, i64)>,
    piece: i64,
    x: i64,
    y: i64,
    rocks: &Vec<Vec<(i64, i64)>>,
) -> bool {
    return rocks[piece as usize]
        .iter()
        .all(|(dx, dy)| free(settled, x + dx, y + dy));
}

fn place(
    settled: &mut BTreeSet<(i64, i64)>,
    jet: i64,
    piece: i64,
    max_y: i64,
    jets: &Vec<char>,
    rocks: &Vec<Vec<(i64, i64)>>,
) -> (i64, i64, i64) {
    let mut x = 2;
    let mut y = max_y + 5;
    let mut new_jet = jet;
    while can_move(settled, piece, x, y - 1, rocks) {
        y -= 1;
        if jets[new_jet as usize] == '<' && can_move(settled, piece, x - 1, y, rocks) {
            x -= 1;
        }
        if jets[new_jet as usize] == '>' && can_move(settled, piece, x + 1, y, rocks) {
            x += 1;
        }
        new_jet = (new_jet + 1) % (jets.len() as i64);
    }
    let new_cells: Vec<(i64, i64)> = rocks[piece as usize]
        .iter()
        .map(|(dx, dy)| (x + dx, y + dy))
        .collect();
    new_cells.iter().for_each(|cell| {
        settled.insert(*cell);
    });
    return (
        new_jet,
        (piece + 1) % rocks.len() as i64,
        cmp::max(max_y, new_cells.iter().map(|(_, y)| *y).max().unwrap()),
    );
}

fn ground_shape(settled: &BTreeSet<(i64, i64)>, max_y: i64) -> Option<Vec<(i64, i64)>> {
    let mut state: BTreeSet<(i64, i64)> = BTreeSet::new();
    for x in 0..WIDTH {
        search(x, 0, &mut state, max_y, settled);
    }
    if state.len() <= CACHE_LEN {
        return Some(state.into_iter().collect::<Vec<(i64, i64)>>());
    } else {
        return None;
    }
}

fn search(
    x: i64,
    y: i64,
    visited: &mut BTreeSet<(i64, i64)>,
    max_y: i64,
    settled: &BTreeSet<(i64, i64)>,
) {
    if (!free(settled, x, max_y + y)) || visited.contains(&(x, y)) || visited.len() > CACHE_LEN {
        return;
    }
    visited.insert((x, y));
    vec![(x - 1, y), (x + 1, y), (x, y - 1)]
        .iter()
        .for_each(|(nx, ny)| {
            search(*nx, *ny, visited, max_y, settled);
        });
}

fn solve(num_rocks: i64, jets: &Vec<char>, rocks: &Vec<Vec<(i64, i64)>>) -> i64 {
    let mut settled: BTreeSet<(i64, i64)> = BTreeSet::new();
    let mut cycles: BTreeMap<(i64, i64, Vec<(i64, i64)>), (i64, i64)> = BTreeMap::new();
    let mut jet = 0;
    let mut max_y = 0;
    let mut piece = 0;
    let mut addl = 0;
    let mut count = num_rocks;

    while count > 0 {
        (jet, piece, max_y) = place(&mut settled, jet, piece, max_y, jets, rocks);
        count -= 1;
        let maybe_ground = ground_shape(&settled, max_y);
        if maybe_ground.is_none() {
            continue;
        }
        let ground = maybe_ground.unwrap();
        if cycles.contains_key(&(jet, piece, ground.clone())) {
            let (old_max_y, old_count) = cycles.get(&(jet, piece, ground.clone())).unwrap();
            addl += (max_y - old_max_y) * (count / (old_count - count));
            count %= old_count - count;
        }
        cycles.insert((jet, piece, ground), (max_y, count));
    }
    return max_y + addl;
}

fn main() {
    let input_str = include_str!("../input.txt");
    let mut jets: Vec<char> = read_input(input_str);
    if jets[jets.len() - 1] == '\n' {
        jets.pop();
    }
    let rocks: Vec<Vec<(i64, i64)>> = vec![
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 0), (0, 1), (1, 0), (1, 1)],
    ];
    println!("Part 1: {}", solve(2022, &jets, &rocks));
    println!("Part 2: {}", solve(1_000_000_000_000, &jets, &rocks));
}
