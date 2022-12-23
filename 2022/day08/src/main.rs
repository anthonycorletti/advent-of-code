fn main() {
    let mut input = include_str!("../input.txt").to_string();
    input.pop(); // Remove trailing newline

    // Parse the input into a 2D vector of i32s
    let mut grid: Vec<Vec<i32>> = Vec::new();
    for line in input.lines() {
        let mut row: Vec<i32> = Vec::new();
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap() as i32);
        }
        grid.push(row);
    }

    // For each tree, if it's visible from any direction, increment the counter
    let mut visible_trees = 0;
    let mut max_vis_score = 0;

    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            let tree_height = grid[row][col];
            let mut visible = 0;
            let mut vis_scorer: Vec<i32> = vec![0; 4];

            // Check from the left
            for i in 0..col {
                if grid[row][i] >= tree_height {
                    visible += 1;
                    break;
                }
            }

            // Check from the right
            for i in col + 1..grid[row].len() {
                if grid[row][i] >= tree_height {
                    visible += 1;
                    break;
                }
            }

            // Check from above
            for i in 0..row {
                if grid[i][col] >= tree_height {
                    visible += 1;
                    break;
                }
            }

            // Check from below
            for i in row + 1..grid.len() {
                if grid[i][col] >= tree_height {
                    visible += 1;
                    break;
                }
            }

            if visible != 4 {
                visible_trees += 1;
            }

            for seek_idx in (0..row).rev() {
                let found = grid[seek_idx][col];
                vis_scorer[0] += 1;
                if found >= tree_height {
                    break;
                }
            }

            for seek_idx in row + 1..grid.len() {
                let found = grid[seek_idx][col];
                vis_scorer[1] += 1;
                if found >= tree_height {
                    break;
                }
            }

            for seek_idx in (0..col).rev() {
                let found = grid[row][seek_idx];
                vis_scorer[2] += 1;
                if found >= tree_height {
                    break;
                }
            }

            for seek_idx in col + 1..grid[row].len() {
                let found = grid[row][seek_idx];
                vis_scorer[3] += 1;
                if found >= tree_height {
                    break;
                }
            }

            let vis_score = vis_scorer.iter().product();
            if vis_score > max_vis_score {
                max_vis_score = vis_score;
            }
        }
    }

    println!("Visible trees: {}", visible_trees);
    println!("Max visibility: {}", max_vis_score)
}
