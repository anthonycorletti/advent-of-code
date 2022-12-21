fn count_until(input: &str, n: usize) -> usize {
    let mut i = 0;

    while i < input.len() - n {
        // increment i if the next 4 characters have any duplicate characters
        if input[i..i + n]
            .chars()
            .any(|c| input[i..i + n].matches(c).count() > 1)
        {
            i += 1;
            continue;
        }
        break;
    }

    n + i as usize
}

fn main() {
    let mut input = include_str!("../input.txt").to_string();
    input.pop(); // remove trailing newline

    let mut i = 0;

    while i < input.len() - 4 {
        // increment i if the next 4 characters have any duplicate characters
        if input[i..i + 4]
            .chars()
            .any(|c| input[i..i + 4].matches(c).count() > 1)
        {
            i += 1;
            continue;
        }
        break;
    }

    println!("{}", count_until(&input, 4));
    println!("{}", count_until(&input, 14));
}
