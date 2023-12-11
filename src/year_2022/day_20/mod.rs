use nom::{
    character::complete::{i64, newline},
    multi::separated_list1,
    IResult,
};
use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::VecDeque;

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
            assert_eq!(result, 3);
        }

        #[test]
        fn it_solves_input() {
            let result = solve_part1(INPUT);
            assert_eq!(result, 11_037);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn it_solves_example() {
            let result = solve_part2(EXAMPLE);
            assert_eq!(result, 1_623_178_306);
        }

        #[test]
        fn it_solves_input() {
            let result = solve_part2(INPUT);
            assert_eq!(result, 3_033_720_253_914);
        }
    }
}

fn line(input: &str) -> IResult<&str, i64> {
    i64(input)
}
fn parse(input: &str) -> Vec<i64> {
    separated_list1(newline, line)(input).unwrap().1
}

fn mix(numbers: &[i64], indices: &mut VecDeque<usize>) {
    for (i, &n) in numbers.iter().enumerate() {
        let i = indices.iter().position(|&x| x == i).unwrap();
        match n.cmp(&0) {
            Less => {
                let value = indices.remove(i).unwrap();
                let rotate_by = n.unsigned_abs() as usize % indices.len();
                indices.rotate_right(rotate_by);
                indices.insert(i, value);
            }
            Greater => {
                let value = indices.remove(i).unwrap();
                let rotate_by = n.unsigned_abs() as usize % indices.len();
                indices.rotate_left(rotate_by);
                indices.insert(i, value);
            }
            Equal => {}
        }
    }
}

pub fn solve_part1(input: &str) -> i64 {
    let numbers = parse(input);

    let mut original_indices: VecDeque<usize> =
        numbers.iter().enumerate().map(|(i, _)| i).collect();
    mix(&numbers, &mut original_indices);

    let zero_idx = numbers.iter().position(|&x| x == 0).unwrap();
    let zero_idx = original_indices
        .iter()
        .position(|&x| x == zero_idx)
        .unwrap();

    [1_000, 2_000, 3_000]
        .iter()
        .map(|i| numbers[original_indices[(zero_idx + i) % numbers.len()]])
        .sum()
}

pub fn solve_part2(input: &str) -> i64 {
    const KEY: i64 = 811_589_153;

    let numbers = parse(input);
    let numbers = numbers.iter().map(|&n| n * KEY).collect::<Vec<_>>();

    let mut original_indices: VecDeque<usize> =
        numbers.iter().enumerate().map(|(i, _)| i).collect();

    for _ in 0..10 {
        mix(&numbers, &mut original_indices);
    }

    let zero_idx = numbers.iter().position(|&x| x == 0).unwrap();
    let zero_idx = original_indices
        .iter()
        .position(|&x| x == zero_idx)
        .unwrap();

    [1000, 2000, 3000]
        .iter()
        .map(|i| numbers[original_indices[(zero_idx + i) % numbers.len()]])
        .sum()
}
