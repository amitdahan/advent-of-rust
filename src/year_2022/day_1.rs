#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
    const INPUT: &str = include_str!("day_1.in");

    mod part1 {
        use super::*;

        #[test]
        fn it_solves_example_input() {
            let result = solve_part1(EXAMPLE_INPUT);
            assert_eq!(result, 24000);
        }

        #[test]
        fn it_solves_input() {
            let result = solve_part1(INPUT);
            assert_eq!(result, 70698);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn it_solves_example() {
            let result = solve_part2(EXAMPLE_INPUT);
            assert_eq!(result, 45000);
        }

        #[test]
        fn it_solves_input() {
            let result = solve_part2(INPUT);
            assert_eq!(result, 206643);
        }
    }
}

pub fn solve_part1(input: &str) -> i32 {
    input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|line| line.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .max()
        .unwrap()
}

pub fn solve_part2(input: &str) -> i32 {
    let mut calories = input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|line| line.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect::<Vec<i32>>();

    calories.sort_by(|a, b| a.cmp(b).reverse());
    calories.truncate(3);

    calories.iter().sum()
}
