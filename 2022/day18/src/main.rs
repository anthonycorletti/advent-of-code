fn main() {
    let input_str = include_str!("../input.txt");
    let content = Input::from_string(input_str);
    println!("Part 1: {}", part1(&content));
    println!("Part 2: {}", part2(&content));
}

fn part1(content: &Input) -> usize {
    let mut exposed = 0;

    for cube in &content.cubes {
        let neighbors = cube.get_neighbors();

        exposed += 6 - content
            .cubes
            .iter()
            .filter(|c| *c != cube)
            .filter(|c| neighbors.contains(c))
            .count();
    }

    exposed
}

fn part2(content: &Input) -> usize {
    let cubes: Vec<Cube> = content
        .cubes
        .iter()
        .map(|c| Cube::new(c.x + 1, c.y + 1, c.z + 1))
        .collect();

    let x = content.cubes.iter().map(|cube| cube.x).max().unwrap() + 2;
    let y = content.cubes.iter().map(|cube| cube.y).max().unwrap() + 2;
    let z = content.cubes.iter().map(|cube| cube.z).max().unwrap() + 2;

    count_faces(&cubes, x, y, z)
}

fn count_faces(cubes: &Vec<Cube>, x: i32, y: i32, z: i32) -> usize {
    let mut queue: Vec<Cube> = vec![Cube::new(0, 0, 0)];
    let mut visited: Vec<Cube> = vec![];
    let mut faces = 0;

    while let Some(cursor) = queue.pop() {
        // get the cells around the cursor
        let around = cursor
            .get_neighbors()
            .into_iter()
            .filter(|cube| {
                cube.x >= 0
                    && cube.y >= 0
                    && cube.z >= 0
                    && cube.x <= x
                    && cube.y <= y
                    && cube.z <= z
            })
            .collect::<Vec<Cube>>();

        // count how many neighbours are actually cubes, these are faces we can count
        faces += around.iter().filter(|c| cubes.contains(c)).count();

        // queue up unvisited neighbours
        let mut next = around
            .into_iter()
            .filter(|c| !cubes.contains(c))
            .filter(|c| !visited.contains(c))
            .filter(|c| !queue.contains(c)) // don't forget the queue or we'll
            .collect(); // visit locations more than once

        queue.append(&mut next);
        visited.push(cursor);
    }

    faces
}

struct Input {
    cubes: Vec<Cube>,
}

impl Input {
    fn from_string(s: &str) -> Self {
        Input {
            cubes: s.lines().map(Cube::from_string).collect(),
        }
    }
}

#[derive(PartialEq)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

impl Cube {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Cube { x, y, z }
    }

    fn from_string(s: &str) -> Self {
        let mut line = s.split(',').into_iter();

        Cube::new(
            line.next().unwrap().parse().unwrap(),
            line.next().unwrap().parse().unwrap(),
            line.next().unwrap().parse().unwrap(),
        )
    }

    fn get_neighbors(&self) -> Vec<Cube> {
        vec![
            Cube::new(self.x + 1, self.y, self.z),
            Cube::new(self.x - 1, self.y, self.z),
            Cube::new(self.x, self.y + 1, self.z),
            Cube::new(self.x, self.y - 1, self.z),
            Cube::new(self.x, self.y, self.z + 1),
            Cube::new(self.x, self.y, self.z - 1),
        ]
    }
}
