use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{char, newline, u8},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
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
            assert_eq!(result, 62);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn it_solves_example() {
            let result = solve_part2(EXAMPLE);
            assert_eq!(result, 952408144115);
        }
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
struct Instruction {
    dir: Direction,
    n: usize,
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    alt((
        map(char('U'), |_| Direction::Up),
        map(char('R'), |_| Direction::Right),
        map(char('D'), |_| Direction::Down),
        map(char('L'), |_| Direction::Left),
    ))(input)
}
fn parse_color(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("(#")(input)?;
    let (input, n) = take(5usize)(input)?;
    let (input, dir) = alt((
        map(char('0'), |_| Direction::Right),
        map(char('1'), |_| Direction::Down),
        map(char('2'), |_| Direction::Left),
        map(char('3'), |_| Direction::Up),
    ))(input)?;
    let (input, _) = char(')')(input)?;

    let n = usize::from_str_radix(n, 16).unwrap();
    Ok((input, Instruction { dir, n }))
}
fn parse_instruction(input: &str) -> IResult<&str, (Instruction, Instruction)> {
    let (input, (dir, (n, instruction2))) = separated_pair(
        parse_direction,
        char(' '),
        separated_pair(u8, char(' '), parse_color),
    )(input)?;
    let instruction1 = Instruction { dir, n: n as usize };
    Ok((input, (instruction1, instruction2)))
}
fn parse(input: &str) -> IResult<&str, Vec<(Instruction, Instruction)>> {
    separated_list1(newline, parse_instruction)(input)
}

fn solve(dig_plan: &[&Instruction]) -> u64 {
    let mut x = 0;
    let mut y = 0;
    let mut points = vec![];
    let mut perimeter = 0;

    for instruction in dig_plan {
        points.push((x, y));
        perimeter += instruction.n as u64;
        match instruction.dir {
            Direction::Up => y += instruction.n as i32,
            Direction::Right => x += instruction.n as i32,
            Direction::Down => y -= instruction.n as i32,
            Direction::Left => x -= instruction.n as i32,
        }
    }

    let area = (points
        .into_iter()
        .circular_tuple_windows()
        .map(|((x1, y1), (x2, y2))| x1 as i64 * y2 as i64 - x2 as i64 * y1 as i64)
        .sum::<i64>()
        / 2)
    .abs() as u64;

    perimeter / 2 + area + 1
}

pub fn solve_part1(input: &str) -> u32 {
    let (_, dig_plan) = parse(input).unwrap();
    solve(&dig_plan.iter().map(|(i, _)| i).collect::<Vec<_>>()) as u32
}

pub fn solve_part2(input: &str) -> u64 {
    let (_, dig_plan) = parse(input).unwrap();
    solve(&dig_plan.iter().map(|(_, i)| i).collect::<Vec<_>>())
}
