use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    combinator::map,
    multi::{many1, separated_list1},
    IResult,
};

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.in");

    mod part1 {
        use super::*;

        #[test]
        fn it_solves_example() {
            let result = solve_part1(EXAMPLE);
            assert_eq!(result, 405);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn it_solves_example() {
            let result = solve_part2(EXAMPLE);
            assert_eq!(result, 400);
        }
    }
}

fn reflection(nums: &[u32], xored_ones: u32) -> Option<usize> {
    (1..nums.len()).find(|&i| {
        nums[0..i]
            .iter()
            .rev()
            .zip(nums[i..nums.len()].iter())
            .map(|(a, b)| (a ^ b).count_ones())
            .sum::<u32>()
            == xored_ones
    })
}
fn horizontal_reflection(map: &Map, xored_ones: u32) -> Option<usize> {
    let mut cols = vec![0; map[0].len()];

    for row in map {
        for (col, cell) in row.iter().enumerate() {
            cols[col] <<= 1;
            cols[col] |= *cell as u32;
        }
    }

    reflection(&cols, xored_ones)
}
fn vertical_reflection(map: &Map, xored_ones: u32) -> Option<usize> {
    let rows = map
        .iter()
        .map(|row| row.iter().fold(0, |acc, cell| (acc << 1) | *cell as u32))
        .collect::<Vec<_>>();

    reflection(&rows, xored_ones)
}

type Map = Vec<Vec<bool>>;

fn parse_map_row(input: &str) -> IResult<&str, Vec<bool>> {
    many1(alt((map(tag("."), |_| false), map(tag("#"), |_| true))))(input)
}
fn parse_map(input: &str) -> IResult<&str, Map> {
    separated_list1(newline, parse_map_row)(input)
}
fn parse_maps(input: &str) -> IResult<&str, Vec<Map>> {
    separated_list1(tag("\n\n"), parse_map)(input)
}

fn solve(input: &str, xored_ones: u32) -> u32 {
    let (_, maps) = parse_maps(input).unwrap();

    maps.iter()
        .map(|map| {
            horizontal_reflection(map, xored_ones)
                .map(|col| col as u32)
                .or(vertical_reflection(map, xored_ones).map(|row| row as u32 * 100))
                .unwrap()
        })
        .sum::<u32>()
}
pub fn solve_part1(input: &str) -> u32 {
    solve(input, 0)
}
pub fn solve_part2(input: &str) -> u32 {
    solve(input, 1)
}
