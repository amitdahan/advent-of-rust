use itertools::Itertools;

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.in");

    mod part1 {
        use super::*;

        #[test]
        fn it_solves_example() {
            let result = solve_part1(EXAMPLE);
            assert_eq!(result, 114);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn it_solves_example() {
            let result = solve_part2(EXAMPLE);
            assert_eq!(result, 2);
        }
    }
}

enum PredictionDirection {
    Next,
    Prev,
}

fn predict(dir: &PredictionDirection, arr: &[i32]) -> i32 {
    if arr.iter().all(|x| *x == 0) {
        return 0;
    }

    let prediction = predict(
        dir,
        &arr.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>(),
    );

    match dir {
        PredictionDirection::Next => arr.last().unwrap() + prediction,
        PredictionDirection::Prev => arr.first().unwrap() - prediction,
    }
}

fn solve(input: &str, dir: &PredictionDirection) -> i32 {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect_vec()
        })
        .map(|arr| predict(dir, &arr))
        .sum()
}

pub fn solve_part1(input: &str) -> i32 {
    solve(input, &PredictionDirection::Next)
}
pub fn solve_part2(input: &str) -> i32 {
    solve(input, &PredictionDirection::Prev)
}
