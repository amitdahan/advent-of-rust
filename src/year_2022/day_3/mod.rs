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
            assert_eq!(result, 157);
        }

        #[test]
        fn it_solves_input() {
            let result = solve_part1(INPUT);
            assert_eq!(result, 7824);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn it_solves_example() {
            let result = solve_part2(EXAMPLE);
            assert_eq!(result, 70);
        }

        #[test]
        fn it_solves_input() {
            let result = solve_part2(INPUT);
            assert_eq!(result, 2798);
        }
    }
}

pub fn solve_part1(input: &str) -> i32 {
    input
        .lines()
        .flat_map(|line| {
            let (first, second) = line.split_at(line.len() / 2);

            let seen_first: HashSet<char> = HashSet::from_iter(first.chars());
            let seen_second: HashSet<char> = HashSet::from_iter(second.chars());

            seen_first
                .intersection(&seen_second)
                .map(|c| -> i32 {
                    if c.is_ascii_lowercase() {
                        *c as i32 - 'a' as i32 + 1
                    } else {
                        *c as i32 - 'A' as i32 + 27
                    }
                })
                .collect::<Vec<i32>>()
        })
        .sum()
}

pub fn solve_part2(input: &str) -> i32 {
    input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .flat_map(|chunk| {
            chunk
                .iter()
                .map(|line| HashSet::<_>::from_iter(line.chars()))
                .reduce(|acc, x| acc.intersection(&x).copied().collect())
                .unwrap()
        })
        .map(|c| -> i32 {
            if c.is_ascii_lowercase() {
                c as i32 - 'a' as i32 + 1
            } else {
                c as i32 - 'A' as i32 + 27
            }
        })
        .sum()
}
