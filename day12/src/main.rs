use std::{
    cmp::{min, Reverse},
    collections::{BinaryHeap, HashMap},
};

#[derive(Clone)]
enum Hill {
    Start(u8),
    End(u8),
    Hill(u8),
}

impl From<char> for Hill {
    fn from(value: char) -> Self {
        match value {
            'S' => Hill::Start(0),
            'E' => Hill::End(25),
            c if c.is_ascii_lowercase() => Hill::Hill(value as u8 - b'a'),
            _ => unreachable!(),
        }
    }
}

impl Hill {
    fn height(&self) -> u8 {
        match self {
            Hill::Start(h) => *h,
            Hill::End(h) => *h,
            Hill::Hill(h) => *h,
        }
    }

    fn can_reach(&self, other: &Hill) -> bool {
        other.height().saturating_sub(self.height()) <= 1
    }
}

type Neighbors = [Option<(usize, usize)>; 4];

struct HillMap {
    hills: Vec<Vec<Hill>>,
    graph: HashMap<(usize, usize), Neighbors>,
    start_at: (usize, usize),
    end_at: (usize, usize),
}

impl From<&str> for HillMap {
    fn from(value: &str) -> Self {
        let hills: Vec<Vec<_>> = value
            .lines()
            .map(|row| row.chars().map(Hill::from).collect())
            .collect();

        let mut graph = HashMap::new();

        let last_row = hills.len().saturating_sub(1);
        let last_col = hills
            .first()
            .map(|r| r.len())
            .unwrap_or_default()
            .saturating_sub(1);

        let mut start_at = (0, 0);
        let mut end_at = (0, 0);

        for (row_idx, row) in hills.iter().enumerate() {
            for (col_idx, hill) in row.iter().enumerate() {
                let mut neighbors = [None; 4];
                if row_idx > 0 && hill.can_reach(&hills[row_idx - 1][col_idx]) {
                    neighbors[0] = Some((row_idx - 1, col_idx));
                }
                if col_idx > 0 && hill.can_reach(&hills[row_idx][col_idx - 1]) {
                    neighbors[1] = Some((row_idx, col_idx - 1));
                }
                if row_idx < last_row && hill.can_reach(&hills[row_idx + 1][col_idx]) {
                    neighbors[2] = Some((row_idx + 1, col_idx));
                }
                if col_idx < last_col && hill.can_reach(&hills[row_idx][col_idx + 1]) {
                    neighbors[3] = Some((row_idx, col_idx + 1));
                }

                if let Hill::Start(_) = hill {
                    start_at = (row_idx, col_idx);
                }
                if let Hill::End(_) = hill {
                    end_at = (row_idx, col_idx);
                }
                graph.insert((row_idx, col_idx), neighbors);
            }
        }

        HillMap {
            hills,
            graph,
            start_at,
            end_at,
        }
    }
}

impl HillMap {
    fn shortest_path_to_summit(&self, start_at: (usize, usize)) -> Option<u32> {
        let mut open = BinaryHeap::from([(Reverse(0), start_at)]);
        let mut steps = HashMap::from([(start_at, 0)]);

        while let Some((_, pos)) = open.pop() {
            let (x, y) = pos;
            if pos == self.end_at {
                return steps.get(&pos).copied();
            }

            let Some(neighbors) = self.graph.get(&pos) else {continue;};

            for _neighbor in neighbors {
                let Some(neighbor) = _neighbor else {continue;};

                let next = steps.get(&pos).unwrap() + 1;

                let curr = *steps.get(neighbor).unwrap_or(&u32::MAX);

                if next >= curr {
                    continue;
                }

                open.push((Reverse(next), *neighbor));
                steps.insert(*neighbor, next);
            }
        }

        None
    }
}

struct DescentMap {
    hills: Vec<Vec<Hill>>,
    graph: HashMap<(usize, usize), Neighbors>,
    summit: (usize, usize),
}

impl From<&HillMap> for DescentMap {
    fn from(hill_map: &HillMap) -> Self {
        let mut graph: HashMap<(usize, usize), Neighbors> = HashMap::new();

        for (pos, neighbors) in hill_map.graph.iter() {
            for neighbor in neighbors.iter().flatten() {
                graph
                    .entry(*neighbor)
                    .or_default()
                    .iter_mut()
                    .filter(|slot| slot.is_none())
                    .take(1)
                    .for_each(|slot| *slot = Some(*pos));
            }
        }

        let hills = hill_map.hills.to_vec();
        let summit: (usize, usize) = hill_map.end_at;

        DescentMap {
            hills,
            graph,
            summit,
        }
    }
}

impl DescentMap {
    fn shortest_path_from_summit(&self) -> HashMap<(usize, usize), u32> {
        let start_at = self.summit;
        let mut open = BinaryHeap::from([(Reverse(0), start_at)]);
        let mut steps = HashMap::from([(start_at, 0)]);

        while let Some((_, pos)) = open.pop() {
            let (x, y) = pos;
            if let Hill::Start(_) = self.hills[x][y] {
                return steps;
            }

            let Some(neighbors) = self.graph.get(&pos) else {continue;};

            for _neighbor in neighbors {
                let Some(neighbor) = _neighbor else {continue;};

                let next = steps.get(&pos).unwrap() + 1;

                let curr = *steps.get(neighbor).unwrap_or(&u32::MAX);

                if next >= curr {
                    continue;
                }

                open.push((Reverse(next), *neighbor));
                steps.insert(*neighbor, next);
            }
        }

        steps
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let hill_map = HillMap::from(input);

    let start_at = hill_map.start_at;

    println!("{}", hill_map.shortest_path_to_summit(start_at).unwrap());

    let descent_map = DescentMap::from(&hill_map);

    let steps = descent_map.shortest_path_from_summit();

    let mut shortest_path = u32::MAX;
    for (pos, steps_to_pos) in steps.iter() {
        let (row, col) = *pos;
        let Hill::Hill(0) = descent_map.hills[row][col] else { continue; };
        shortest_path = min(shortest_path, *steps_to_pos);
    }

    // Return the shortest path to a short hill
    println!("{}", shortest_path.to_string())
}
