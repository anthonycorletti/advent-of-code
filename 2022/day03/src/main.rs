use std::collections::HashSet;

fn char_to_code(c: char) -> i32 {
    let code = c as i32;
    if code >= 97 && code <= 122 {
        code - 96
    } else {
        code - 38
    }
}

fn main() {
    let mut input = include_str!("../input.txt").to_string();
    input.pop(); // remove the last newline
    let mut lines: Vec<&str> = input.split("\n").collect();
    let mut sum: i32 = 0;
    let mut sum_2: i32 = 0;

    for (i, line) in lines.iter().enumerate() {
        // skip empty lines
        if line.len() == 0 {
            continue;
        }
        // split the line in half
        let (left, right) = line.split_at(line.len() / 2);

        // create a set of chars for both halves
        let left_chars: HashSet<_> = left.chars().collect();
        let right_chars: HashSet<_> = right.chars().collect();

        // find the intersection of the two sets
        let intersection: HashSet<_> = left_chars.intersection(&right_chars).collect();

        let c = intersection.iter().next().unwrap();

        sum += char_to_code(**c);
    }

    // PART 2
    for i in (0..lines.len()).step_by(3) {
        let line_1 = lines[i];
        let line_2 = lines[i + 1];
        let line_3 = lines[i + 2];

        for c in line_1.chars() {
            if line_2.contains(c) && line_3.contains(c) {
                sum_2 += char_to_code(c);
                break
            }
        }
    }

    println!("sum: {}", sum);
    println!("sum_2: {}", sum_2);
}
