use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = include_str!("example_1.in");
    const EXAMPLE_2: &str = include_str!("example_2.in");
    const EXAMPLE_3: &str = include_str!("example_3.in");
    const EXAMPLE_4: &str = include_str!("example_4.in");
    const EXAMPLE_5: &str = include_str!("example_5.in");

    const INPUT: &str = include_str!("input.in");

    mod part1 {
        use super::*;

        #[test]
        fn it_solves_example_1() {
            let result = solve_part1(EXAMPLE_1);
            assert_eq!(result, 5);
        }

        #[test]
        fn it_solves_example_2() {
            let result = solve_part1(EXAMPLE_2);
            assert_eq!(result, 6);
        }

        #[test]
        fn it_solves_example_3() {
            let result = solve_part1(EXAMPLE_3);
            assert_eq!(result, 10);
        }

        #[test]
        fn it_solves_example_4() {
            let result = solve_part1(EXAMPLE_4);
            assert_eq!(result, 11);
        }

        #[test]
        fn it_solves_example_5() {
            let result = solve_part1(EXAMPLE_5);
            assert_eq!(result, 7);
        }

        #[test]
        fn it_solves_input() {
            let result = solve_part1(INPUT);
            assert_eq!(result, 1238);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn it_solves_example_1() {
            let result = solve_part2(EXAMPLE_1);
            assert_eq!(result, 23);
        }

        #[test]
        fn it_solves_example_2() {
            let result = solve_part2(EXAMPLE_2);
            assert_eq!(result, 23);
        }

        #[test]
        fn it_solves_example_3() {
            let result = solve_part2(EXAMPLE_3);
            assert_eq!(result, 29);
        }

        #[test]
        fn it_solves_example_4() {
            let result = solve_part2(EXAMPLE_4);
            assert_eq!(result, 26);
        }

        #[test]
        fn it_solves_example_5() {
            let result = solve_part2(EXAMPLE_5);
            assert_eq!(result, 19);
        }

        #[test]
        fn it_solves_input() {
            let result = solve_part2(INPUT);
            assert_eq!(result, 3037);
        }
    }
}

pub fn solve_part1(input: &str) -> i32 {
    solve_n(input, 4)
}

pub fn solve_part2(input: &str) -> i32 {
    solve_n(input, 14)
}

fn solve_n(input: &str, n: usize) -> i32 {
    let mut char_counts = HashMap::<char, usize>::new();

    let chars = input.chars().collect::<Vec<_>>();

    for (i, c) in chars.iter().enumerate() {
        *char_counts.entry(*c).or_insert(0) += 1;

        if i >= n {
            *char_counts.entry(chars[i - n]).or_insert(0) -= 1;
            if char_counts[&chars[i - n]] == 0 {
                char_counts.remove(&chars[i - n]);
            }
        }

        if char_counts.keys().count() == n {
            return (i + 1) as i32;
        }
    }

    -1
}
