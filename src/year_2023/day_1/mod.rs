#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = include_str!("example_1.in");
    const EXAMPLE_2: &str = include_str!("example_2.in");
    const INPUT: &str = include_str!("input.in");

    mod part1 {
        use super::*;

        #[test]
        fn it_solves_example() {
            let result = solve_part1(EXAMPLE_1);
            assert_eq!(result, 142);
        }

        #[test]
        fn it_solves_input() {
            let result = solve_part1(INPUT);
            assert_eq!(result, 54968);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn it_solves_example() {
            let result = solve_part2(EXAMPLE_2);
            assert_eq!(result, 281);
        }

        #[test]
        fn it_solves_input() {
            let result = solve_part2(INPUT);
            assert_eq!(result, 54094);
        }
    }
}

pub fn solve_part1(input: &str) -> u32 {
    let mut sum = 0;

    for row in input.lines() {
        let first_digit = row
            .chars()
            .find(|c| c.is_numeric())
            .map(|c| c.to_digit(10))
            .unwrap()
            .unwrap();
        let last_digit = row
            .chars()
            .rev()
            .find(|c| c.is_numeric())
            .map(|c| c.to_digit(10))
            .unwrap()
            .unwrap();

        sum += first_digit * 10 + last_digit;
    }

    sum
}

const DIGIT_WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn solve_part2(input: &str) -> u32 {
    let mut sum = 0;

    for row in input.lines() {
        let first_digit = row
            .char_indices()
            .find_map(|(i, c)| {
                if c.is_numeric() {
                    Some(c.to_digit(10).unwrap())
                } else {
                    DIGIT_WORDS
                        .iter()
                        .enumerate()
                        .find_map(|(digit_i, digit_str)| {
                            if row.get(i..).unwrap().starts_with(digit_str) {
                                Some(digit_i as u32 + 1)
                            } else {
                                None
                            }
                        })
                }
            })
            .unwrap();

        let last_digit = row
            .char_indices()
            .rev()
            .find_map(|(i, c)| {
                if c.is_numeric() {
                    Some(c.to_digit(10).unwrap())
                } else {
                    DIGIT_WORDS
                        .iter()
                        .enumerate()
                        .find_map(|(digit_i, digit_str)| {
                            if row.get(..=i).unwrap().ends_with(digit_str) {
                                Some(digit_i as u32 + 1)
                            } else {
                                None
                            }
                        })
                }
            })
            .unwrap();
        sum += first_digit * 10 + last_digit;
    }

    sum
}
