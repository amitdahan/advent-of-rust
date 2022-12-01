#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn it_solves_example() {
            let result = solve_part1(
                "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000",
            );
            assert_eq!(result, 24000);
        }

        #[test]
        fn it_solves_input() {
            let result = solve_part1(include_str!("day_1.in"));
            assert_eq!(result, 70698);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn it_solves_example() {
            let result = solve_part2(
                "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000",
            );
            assert_eq!(result, 45000);
        }

        #[test]
        fn it_solves_input() {
            let result = solve_part2(include_str!("day_1.in"));
            assert_eq!(result, -1);
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
