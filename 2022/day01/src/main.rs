pub fn main() {
    let input: Vec<String> = include_str!("../input.txt").lines()
        .map(|l| l.to_string())
        .collect();


    let mut sums: Vec<i32> = Vec::new();
    let mut cur_sum: i32 = 0;

    for el in input {
        if el == "" {
            sums.push(cur_sum);
            cur_sum = 0;
        } else {
            let num: i32 = el.parse().unwrap();
            cur_sum += num;
        }
    }

    sums.sort_by(|a, b| b.cmp(a));
    println!("{:?}", &sums[0..3]);
    let sum: i32 = sums[0..3].iter().sum();
    println!("{}", sum);
}
