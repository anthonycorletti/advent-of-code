use regex::Regex;

struct Procedure {
    n: i32,
    from: usize,
    to: usize,
}

fn main() {
    let mut input = include_str!("../input.txt").to_string();
    input.pop(); // remove trailing newline

    // build stacks
    let stacks_procedures_input: Vec<&str> = input.split("\n\n").collect();
    let stacks_str: Vec<&str> = stacks_procedures_input[0].split("\n").collect();
    let stacks_height = stacks_str.len() - 1;
    let num_stacks = (stacks_str.last().unwrap().len() + 1) / 4;
    let mut stacks: Vec<Vec<char>> = vec![vec![]; num_stacks];
    let stack_regex = Regex::new(r"\[[A-Z]\]|    ").unwrap();
    let crate_regex = Regex::new(r"[A-Z]").unwrap();
    for stack_height_index in (0..stacks_height).rev() {
        let stack_height = stacks_str[stack_height_index];
        let mut s_index = 0;
        for capture in stack_regex.captures_iter(stack_height) {
            if capture[0].starts_with(" ") {
                s_index += 1;
                continue;
            }
            let container = &crate_regex.captures(&capture[0]).unwrap()[0];
            stacks[s_index].push(container.chars().next().unwrap());
            s_index += 1;
        }
    }

    // build procedures
    let procedures_str: Vec<&str> = stacks_procedures_input[1].split("\n").collect();
    let procedures: Vec<Procedure> = procedures_str
        .iter()
        .map(|procedure| {
            let procedure_split: Vec<&str> = procedure.split(" ").collect();
            let n = procedure_split[1].parse::<i32>().unwrap();
            let from = procedure_split[3].parse::<usize>().unwrap() - 1;
            let to = procedure_split[5].parse::<usize>().unwrap() - 1;
            Procedure { n, from, to }
        })
        .collect();

    // execute procedures
    let mut _stacks = stacks.clone();
    for procedure in &procedures {
        for _ in 0..procedure.n {
            let container = _stacks[procedure.from as usize].pop().unwrap();
            _stacks[procedure.to as usize].push(container);
        }
    }

    // print the top of each stack
    let mut solution = "".to_owned();
    for stack in _stacks {
        solution.push(*stack.last().unwrap());
    }

    println!("Part 1: {}", solution);

    for procedure in &procedures {
        let mut container_group: Vec<char> = vec![];
        for _ in 0..procedure.n {
            container_group.insert(0, stacks[procedure.from].pop().unwrap());
        }
        stacks[procedure.to].append(&mut container_group);
    }

    // print the top of each stack
    let mut solution = "".to_owned();
    for stack in stacks {
        solution.push(*stack.last().unwrap());
    }

    println!("Part 2: {}", solution);
}
