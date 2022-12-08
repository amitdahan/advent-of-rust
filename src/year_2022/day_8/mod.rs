use std::collections::HashSet;

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.in");
    const INPUT: &str = include_str!("input.in");

    mod part1 {
        use super::*;

        #[test]
        fn it_solves_example() {
            let result = solve_part1(EXAMPLE);
            assert_eq!(result, 21);
        }

        #[test]
        fn it_solves_input() {
            let result = solve_part1(INPUT);
            assert_eq!(result, 1851);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn it_solves_example() {
            let result = solve_part2(EXAMPLE);
            assert_eq!(result, 8);
        }

        #[test]
        fn it_solves_input() {
            let result = solve_part2(INPUT);
            assert_eq!(result, 574080);
        }
    }
}

fn parse_trees(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect()
}

pub fn solve_part1(input: &str) -> i32 {
    let mat = parse_trees(input);

    // Keys are `row_idx,col_idx` strings
    let mut counted: HashSet<String> = HashSet::new();

    for row_idx in 0..mat.len() {
        let row = &mat[row_idx];
        let col_idx = 0;
        let mut row_max = row[col_idx];
        counted.insert(format!("{},{}", row_idx, col_idx));

        for col_idx in 1..row.len() {
            let cell = row[col_idx];
            if cell > row_max {
                row_max = cell;
                counted.insert(format!("{},{}", row_idx, col_idx));
            }
        }
    }

    for row_idx in 0..mat.len() {
        let row = &mat[row_idx];
        let col_idx = row.len() - 1;
        let mut row_max = row[col_idx];
        counted.insert(format!("{},{}", row_idx, col_idx));

        for col_idx in (0..row.len() - 1).rev() {
            let cell = row[col_idx];
            if cell > row_max {
                row_max = cell;
                counted.insert(format!("{},{}", row_idx, col_idx));
            }
        }
    }

    for col_idx in 0..mat[0].len() {
        let row_idx = 0;
        let mut col_max = mat[row_idx][col_idx];
        counted.insert(format!("{},{}", row_idx, col_idx));

        for row_idx in 1..mat.len() {
            let cell = mat[row_idx][col_idx];
            if cell > col_max {
                col_max = cell;
                counted.insert(format!("{},{}", row_idx, col_idx));
            }
        }
    }

    for col_idx in 0..mat[0].len() {
        let row_idx = mat.len() - 1;
        let mut col_max = mat[row_idx][col_idx];
        counted.insert(format!("{},{}", row_idx, col_idx));

        for row_idx in (0..mat.len() - 1).rev() {
            let cell = mat[row_idx][col_idx];
            if cell > col_max {
                col_max = cell;
                counted.insert(format!("{},{}", row_idx, col_idx));
            }
        }
    }

    counted.len() as i32
}

pub fn solve_part2(input: &str) -> i32 {
    let mat = parse_trees(input);

    let mut max_scenic_score = 0;

    for row_idx in 0..mat.len() {
        let row = &mat[row_idx];

        for col_idx in 1..row.len() {
            let cell = row[col_idx];

            let mut top_distance = 0;
            for top_row_idx in (0..row_idx).rev() {
                top_distance += 1;
                if mat[top_row_idx][col_idx] >= cell {
                    break;
                }
            }

            let mut bottom_distance = 0;
            for bottom_row_idx in row_idx + 1..mat.len() {
                bottom_distance += 1;
                if mat[bottom_row_idx][col_idx] >= cell {
                    break;
                }
            }

            let mut left_distance = 0;
            for left_col_idx in (0..col_idx).rev() {
                left_distance += 1;
                if row[left_col_idx] >= cell {
                    break;
                }
            }

            let mut right_distance = 0;
            for right_col_idx in col_idx + 1..row.len() {
                right_distance += 1;
                if row[right_col_idx] >= cell {
                    break;
                }
            }

            let scenic_score = top_distance * bottom_distance * left_distance * right_distance;

            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score;
            }
        }
    }

    max_scenic_score
}
