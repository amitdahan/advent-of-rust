use adjacent_pair_iterator::AdjacentPairIterator;

#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn it_solves_example_1() {
            let input = "199
200
208
210
200
207
240
269
260
263";

            let result = solve_part1(input);
            assert_eq!(result, 7);
        }

        #[test]
        fn it_solves_input_1() {
            let input = include_str!("day_1.in");

            let result = solve_part1(input);
            assert_eq!(result, 1301);
        }
    }
}

pub fn solve_part1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .adjacent_pairs()
        .filter(|(a, b)| b > a)
        .count() as i32
}
