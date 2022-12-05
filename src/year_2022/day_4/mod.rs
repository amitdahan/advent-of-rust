use std::ops::RangeInclusive;

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
            assert_eq!(result, 2);
        }

        #[test]
        fn it_solves_input() {
            let result = solve_part1(INPUT);
            assert_eq!(result, 424);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn it_solves_example() {
            let result = solve_part2(EXAMPLE);
            assert_eq!(result, 4);
        }

        #[test]
        fn it_solves_input() {
            let result = solve_part2(INPUT);
            assert_eq!(result, 804);
        }
    }
}

fn contains(a: &RangeInclusive<i32>, b: &RangeInclusive<i32>) -> bool {
    a.start() <= b.start() && b.end() <= a.end() || b.start() <= a.start() && a.end() <= b.end()
}

fn overlaps(a: &RangeInclusive<i32>, b: &RangeInclusive<i32>) -> bool {
    a.start() <= b.start() && b.start() <= a.end() || b.start() <= a.start() && a.start() <= b.end()
}

fn parse_elf_range(range: &str) -> RangeInclusive<i32> {
    range
        .split_once('-')
        .map(|(a, b)| {
            let a = a.parse::<i32>().unwrap();
            let b = b.parse::<i32>().unwrap();
            a..=b
        })
        .unwrap()
}

fn parse_input_line(line: &str) -> (RangeInclusive<i32>, RangeInclusive<i32>) {
    line.split_once(',')
        .map(|(a, b)| {
            let a = parse_elf_range(a);
            let b = parse_elf_range(b);
            (a, b)
        })
        .unwrap()
}

pub fn solve_part1(input: &str) -> i32 {
    input
        .lines()
        .map(parse_input_line)
        .filter(|(a, b)| contains(a, b))
        .count() as i32
}

pub fn solve_part2(input: &str) -> i32 {
    input
        .lines()
        .map(parse_input_line)
        .filter(|(a, b)| overlaps(a, b))
        .count() as i32
}
