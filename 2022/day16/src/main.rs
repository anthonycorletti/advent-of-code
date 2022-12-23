use core::num;

fn main() {
    let contents = include_str!("../input.txt");
    let input = Input::from(contents);
    println!("Part 1: {}", part1(&input));
    println!("Part 1: {}", part2(&input));
}

struct Input {
    valves: Vec<Valve>,
}

impl Input {
    fn from(contents: &str) -> Self {
        Input::from_string(contents.trim())
    }

    fn from_string(contents: &str) -> Self {
        Input {
            valves: contents.lines().map(Valve::from_string).collect(),
        }
    }
}

struct Valve {
    index: usize,
    flow_rate: u32,
    tunnels: Vec<usize>,
}

impl Valve {
    fn from_string(s: &str) -> Self {
        let re =
            regex::Regex::new(r"Valve (..) has flow rate=(\d+); tunnels? leads? to valves? (.+)$")
                .unwrap();
        let captures = re.captures(s).unwrap();

        Valve {
            index: Valve::index_from(&captures[1]),
            flow_rate: captures[2].parse().unwrap(),
            tunnels: Valve::tunnels_from(&captures[3]),
        }
    }

    fn index_from(s: &str) -> usize {
        let chars = s.as_bytes();
        (chars[0] - b'A') as usize * 26 + (chars[1] - b'A') as usize
    }

    fn tunnels_from(s: &str) -> Vec<usize> {
        s.split(", ").map(Valve::index_from).collect()
    }
}

type DistanceGrid = Vec<Vec<u32>>;

fn part1(input: &Input) -> u32 {
    let distance_grid = build_distance_grid(&input.valves);
    let closed_valves = input
        .valves
        .iter()
        .filter(|valve| valve.flow_rate > 0)
        .collect();
    let start = Valve::index_from("AA");
    let num_minutes = 30;
    let total_pressure = run(&distance_grid, closed_valves, start, num_minutes);
    total_pressure
}

fn part2(input: &Input) -> u32 {
    let distance_grid = build_distance_grid(&input.valves);
    let valves = input
        .valves
        .iter()
        .filter(|v| v.flow_rate > 0)
        .collect::<Vec<&Valve>>();
    let count = valves.len();

    assert!(count <= 16);

    let last_index = 2_u32.pow(count as u32);

    // phase 1: compute and store the result of best_path() for each possible subset of valves
    let mut pressures: Vec<u32> = vec![0; last_index as usize];

    for i in 1..last_index {
        let valve_set: Vec<&Valve> = get_valves_for_bitstring(i, count, &valves);
        let pressure = run(&distance_grid, valve_set, Valve::index_from("AA"), 30 - 4); // subtract 4 minutes to train the elephant

        pressures[i as usize] = pressure;
    }

    // phase 2: find the best pressure possible when adding the pressure from one set of
    // valves to its *complement* set of valves. this accounts for both us and the elephant
    let mut best = 0;

    for us in 1..last_index {
        let elephant = bitstring_complement(us, count as u32);
        let sum = pressures[us as usize] + pressures[elephant as usize];

        best = best.max(sum)
    }

    best
}

fn run(
    distance_grid: &DistanceGrid,
    closed_valves: Vec<&Valve>,
    start: usize,
    num_minutes: u32,
) -> u32 {
    let mut pressures: Vec<u32> = vec![];

    for valve in &closed_valves {
        let distance = distance_grid[start][valve.index];

        if distance >= num_minutes {
            continue;
        }

        let pressure = valve.flow_rate * (num_minutes - distance - 1);

        let remaining = closed_valves
            .iter()
            .filter(|v| v.index != valve.index)
            .cloned()
            .collect();

        let best_pressure = pressure
            + run(
                distance_grid,
                remaining,
                valve.index,
                num_minutes - distance - 1,
            );

        pressures.push(best_pressure);
    }

    pressures.into_iter().max().unwrap_or(0)
}

fn build_distance_grid(valves: &[Valve]) -> DistanceGrid {
    let last_index = Valve::index_from("ZZ");
    let mut edges = vec![vec![]; last_index + 1];
    let vertices: Vec<usize> = valves.iter().map(|valve| valve.index).collect();

    for valve in valves {
        for tunnel in &valve.tunnels {
            edges[valve.index].push(*tunnel);
        }
    }

    let mut distance_grid = vec![vec![0; last_index + 1]; last_index + 1];
    for valve in valves {
        let distances = dijkstra(&edges, &vertices, last_index, valve.index);
        distance_grid[valve.index] = distances;
    }

    distance_grid
}

fn dijkstra(edges: &[Vec<usize>], vertices: &[usize], last_index: usize, start: usize) -> Vec<u32> {
    // initialize grid of "infinite" distances
    let mut distance_to: Vec<u32> = vec![u32::MAX - 1; last_index + 1];

    // queue up every coordinate
    use std::collections::HashSet;
    let mut queue: HashSet<usize> = vertices.iter().cloned().collect();

    // set the first known distance: 0 from the start to the start
    distance_to[start] = 0;

    while !queue.is_empty() {
        // find the position in the queue with shortest distance from the starting valve
        let u = *queue
            .iter()
            .min_by(|&&a, &&b| distance_to[a].cmp(&distance_to[b]))
            .unwrap();

        queue.remove(&u);

        // get all valves adjacent to the starting one that are still in the queue
        let neighbours: Vec<usize> = edges[u]
            .iter()
            .filter(|valve| queue.contains(valve))
            .cloned()
            .collect();

        for v in neighbours {
            // a step to a neighbouring valve is always a distance of 1 (otherwise
            // we would've had to pass in edge weights to this function as well)
            let alt = distance_to[u] + 1;

            if alt < distance_to[v] {
                distance_to[v] = alt;
            }
        }
    }

    distance_to
}

// if the bitstring is 5 (binary 101) then include the 1st and 3rd valves
fn get_valves_for_bitstring<'a>(
    bitstring: u32,
    count: usize,
    valves: &[&'a Valve],
) -> Vec<&'a Valve> {
    get_elements_for_bitstring(bitstring, count, valves)
}

fn get_elements_for_bitstring<'a, T>(
    bitstring: u32,
    count: usize,
    elements: &[&'a T],
) -> Vec<&'a T> {
    let mut vec: Vec<&T> = Vec::with_capacity(count);

    for (i, el) in elements.iter().enumerate() {
        let anded = bitstring & 2_u32.pow(i as u32);
        if anded > 0 {
            vec.push(el);
        }
    }

    vec
}

// 011000 -> 100111
fn bitstring_complement(num: u32, bit_count: u32) -> u32 {
    !(num as u32) & (2_u32.pow(bit_count) - 1)
}
