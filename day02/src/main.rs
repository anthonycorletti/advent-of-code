enum MovePoints {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

enum OutcomePoints {
    Win = 6,
    Draw = 3,
    Lose = 0,
}

fn main() {
    let input = include_str!("../input.txt");
    let mut points = 0;
    let mut points_part_2 = 0;

    for line in input.lines() {
        let mut moves = line.split_whitespace();
        let p1 = moves.next().unwrap();
        let p2 = moves.next().unwrap();

        let p1_points = match p1 {
            "A" => MovePoints::Rock,
            "B" => MovePoints::Paper,
            "C" => MovePoints::Scissors,
            _ => panic!("Invalid move"),
        };

        let p2_points = match p2 {
            "X" => MovePoints::Rock,
            "Y" => MovePoints::Paper,
            "Z" => MovePoints::Scissors,
            _ => panic!("Invalid move"),
        };

        let p2_points_2: OutcomePoints = match p2 {
            "X" => OutcomePoints::Lose,
            "Y" => OutcomePoints::Draw,
            "Z" => OutcomePoints::Win,
            _ => panic!("Invalid move"),
        };

        let outcome = match p2_points {
            MovePoints::Rock => match p1_points {
                MovePoints::Rock => OutcomePoints::Draw,
                MovePoints::Paper => OutcomePoints::Lose,
                MovePoints::Scissors => OutcomePoints::Win,
            },
            MovePoints::Paper => match p1_points {
                MovePoints::Rock => OutcomePoints::Win,
                MovePoints::Paper => OutcomePoints::Draw,
                MovePoints::Scissors => OutcomePoints::Lose,
            },
            MovePoints::Scissors => match p1_points {
                MovePoints::Rock => OutcomePoints::Lose,
                MovePoints::Paper => OutcomePoints::Win,
                MovePoints::Scissors => OutcomePoints::Draw,
            },
        };

        let outcome_2 = match p2_points_2 {
            OutcomePoints::Win => match p1_points {
                MovePoints::Rock => MovePoints::Paper,
                MovePoints::Paper => MovePoints::Scissors,
                MovePoints::Scissors => MovePoints::Rock,
            },
            OutcomePoints::Draw => match p1_points {
                MovePoints::Rock => MovePoints::Rock,
                MovePoints::Paper => MovePoints::Paper,
                MovePoints::Scissors => MovePoints::Scissors,
            },
            OutcomePoints::Lose => match p1_points {
                MovePoints::Rock => MovePoints::Scissors,
                MovePoints::Paper => MovePoints::Rock,
                MovePoints::Scissors => MovePoints::Paper,
            },
        };

        points += p2_points as i32;
        points += outcome as i32;

        points_part_2 += p2_points_2 as i32;
        points_part_2 += outcome_2 as i32;
    }

    println!("{}", points);
    println!("{}", points_part_2);
}
