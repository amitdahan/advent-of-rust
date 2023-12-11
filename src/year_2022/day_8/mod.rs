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

    let mut counted = HashSet::new();

    for (row_idx, row) in mat.iter().enumerate() {
        let mut row_max = row[0];
        counted.insert((row_idx, 0usize));

        for (col_idx, &cell) in row.iter().enumerate().skip(1) {
            if cell > row_max {
                row_max = cell;
                counted.insert((row_idx, col_idx));
            }
        }
    }

    for (row_idx, row) in mat.iter().enumerate() {
        let col_idx = row.len() - 1;
        let mut row_max = row[col_idx];
        counted.insert((row_idx, col_idx));

        for (col_idx, &cell) in row.iter().enumerate().rev().skip(1) {
            if cell > row_max {
                row_max = cell;
                counted.insert((row_idx, col_idx));
            }
        }
    }

    for (col_idx, _) in mat[0].iter().enumerate() {
        let mut col_max = mat[0][col_idx];
        counted.insert((0usize, col_idx));

        for (row_idx, row) in mat.iter().enumerate().skip(1) {
            let cell = row[col_idx];
            if cell > col_max {
                col_max = cell;
                counted.insert((row_idx, col_idx));
            }
        }
    }

    for (col_idx, _) in mat[0].iter().enumerate() {
        let mut col_max = mat[mat.len() - 1][col_idx];
        counted.insert((mat.len() - 1, col_idx));

        for (row_idx, row) in mat.iter().enumerate().rev().skip(1) {
            let cell = row[col_idx];
            if cell > col_max {
                col_max = cell;
                counted.insert((row_idx, col_idx));
            }
        }
    }

    counted.len() as i32
}

pub fn solve_part2(input: &str) -> i32 {
    let mat = parse_trees(input);

    let mut max_scenic_score = 0;

    for (row_idx, row) in mat.iter().enumerate() {
        for (col_idx, _) in row.iter().enumerate().skip(1) {
            let cell = row[col_idx];

            let mut top_distance = 0;
            for row in mat.iter().take(row_idx).rev() {
                top_distance += 1;
                if row[col_idx] >= cell {
                    break;
                }
            }

            let mut bottom_distance = 0;
            for row in mat.iter().skip(row_idx + 1) {
                bottom_distance += 1;
                if row[col_idx] >= cell {
                    break;
                }
            }

            let mut left_distance = 0;
            for &cell2 in row.iter().take(col_idx).rev() {
                left_distance += 1;
                if cell2 >= cell {
                    break;
                }
            }

            let mut right_distance = 0;
            for &cell2 in row.iter().skip(col_idx + 1) {
                right_distance += 1;
                if cell2 >= cell {
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
