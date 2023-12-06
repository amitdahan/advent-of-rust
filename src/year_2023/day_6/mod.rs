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
            assert_eq!(result, 288);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn it_solves_example() {
            let result = solve_part2(EXAMPLE);
            assert_eq!(result, 71503);
        }
    }
}

#[derive(Debug)]
struct Round {
    time: u64,
    record: u64,
}

fn parse_input_1(input: &str) -> Vec<Round> {
    let (time, record) = input.lines().next_tuple().unwrap();
    time.split_whitespace()
        .zip(record.split_whitespace())
        .skip(1)
        .map(|(time, record)| Round {
            time: time.parse().unwrap(),
            record: record.parse().unwrap(),
        })
        .collect()
}

fn parse_input_2(input: &str) -> Round {
    let (time, record) = input.lines().next_tuple().unwrap();
    let time = time.split_whitespace().skip(1).join("").parse().unwrap();
    let record = record.split_whitespace().skip(1).join("").parse().unwrap();
    Round { time, record }
}

fn count_wins(Round { time, record }: &Round) -> u64 {
    let is_winner = |speed: &_| (speed * (time - speed)) > *record;
    let min_hold_time = (0..=*time).find(is_winner).unwrap();
    let max_hold_time = (0..=*time).rfind(is_winner).unwrap();
    max_hold_time - min_hold_time + 1
}

pub fn solve_part1(input: &str) -> u64 {
    parse_input_1(input).iter().map(count_wins).product()
}

pub fn solve_part2(input: &str) -> u64 {
    count_wins(&parse_input_2(input))
}
