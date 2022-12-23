fn main() {
    let input = Input::from("./input.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

struct Input {
    sequence: Vec<i64>,
}

impl Input {
    fn from(file: &str) -> Self {
        let contents = std::fs::read_to_string(file).expect("Couldn't read input");
        Input::from_string(contents.trim())
    }

    fn from_string(s: &str) -> Self {
        Input {
            sequence: s.lines().map(|l| l.parse().unwrap()).collect(),
        }
    }
}

fn part1(input: &Input) -> i64 {
    decrypt(&input.sequence, 1, 1)
}

fn part2(input: &Input) -> i64 {
    decrypt(&input.sequence, 811589153, 10)
}

#[derive(Clone)]
struct Number {
    original_index: usize,
    move_by: i64,
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        self.original_index == other.original_index
    }
}

fn decrypt(numbers: &[i64], decryption_key: i64, iterations: usize) -> i64 {
    // assign the index of each number in the input and multiply by the decryption key
    let mut sequence: Vec<Number> = numbers
        .iter()
        .enumerate()
        .map(|(original_index, &move_by)| Number {
            original_index,
            move_by: move_by * decryption_key,
        })
        .collect();

    let len = sequence.len();

    for _ in 0..iterations {
        for i in 0..len {
            let index = sequence
                .iter()
                .position(|num| num.original_index == i)
                .unwrap();
            let offset = sequence[index].move_by;
            let shifted = shift_element(&sequence, index, offset);
            sequence = shifted;
        }
    }

    // find 0
    let index_zero = sequence.iter().position(|num| num.move_by == 0).unwrap();

    sequence[(index_zero + 1000) % len].move_by
        + sequence[(index_zero + 2000) % len].move_by
        + sequence[(index_zero + 3000) % len].move_by
}

// this is a functional way to move an element in a cycle to some other place in the cycle.
// it wastes a bit of space but it's conceptually easier to understand I think
fn shift_element<T>(vec: &Vec<T>, index: usize, offset: i64) -> Vec<T>
where
    T: Clone + PartialEq,
{
    let len = vec.len() as i64;

    // line up three copies of the vector
    let tripled = [vec.clone(), vec.clone(), vec.clone()].concat();

    // wrap the offset to somewhere in our tripled vector
    let offset = (offset % (len - 1) + len) % len + if offset > 0 { 1 } else { 0 };

    [
        // put the indexed value at the front of the new vector
        vec![vec[index].clone()],
        // pull the rest of the cycle, making sure to filter out the one we put at the front
        tripled
            .into_iter()
            .skip(index + offset as usize)
            .filter(|el| *el != vec[index])
            .take(vec.len() - 1)
            .collect(),
    ]
    .concat()
}
