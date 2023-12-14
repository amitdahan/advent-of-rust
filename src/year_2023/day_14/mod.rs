use std::collections::HashMap;

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
            assert_eq!(result, 136);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn it_solves_example() {
            let result = solve_part2(EXAMPLE);
            assert_eq!(result, 64);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
enum Cell {
    Empty,
    RoundRock,
    CubeRock,
}

fn parse_map_row(input: &str) -> IResult<&str, Vec<Cell>> {
    many1(alt((
        map(tag("."), |_| Cell::Empty),
        map(tag("#"), |_| Cell::CubeRock),
        map(tag("O"), |_| Cell::RoundRock),
    )))(input)
}
fn parse_map(input: &str) -> IResult<&str, Vec<Vec<Cell>>> {
    separated_list1(newline, parse_map_row)(input)
}

fn tilt_north(map: &mut Vec<Vec<Cell>>) {
    let width = map[0].len();
    let height = map.len();
    let mut moved = true;

    while moved {
        moved = false;

        for y in 1..height {
            for x in 0..width {
                if let Cell::RoundRock = map[y][x] {
                    if let Cell::Empty = map[y - 1][x] {
                        map[y][x] = Cell::Empty;
                        map[y - 1][x] = Cell::RoundRock;
                        moved = true;
                    }
                }
            }
        }
    }
}
fn tilt_west(map: &mut Vec<Vec<Cell>>) {
    let width = map[0].len();
    let height = map.len();
    let mut moved = true;

    while moved {
        moved = false;

        for x in 1..width {
            for y in 0..height {
                if let Cell::RoundRock = map[y][x] {
                    if let Cell::Empty = map[y][x - 1] {
                        map[y][x] = Cell::Empty;
                        map[y][x - 1] = Cell::RoundRock;
                        moved = true;
                    }
                }
            }
        }
    }
}
fn tilt_south(map: &mut Vec<Vec<Cell>>) {
    let width = map[0].len();
    let height = map.len();
    let mut moved = true;

    while moved {
        moved = false;

        for y in (0..height - 1).rev() {
            for x in 0..width {
                if let Cell::RoundRock = map[y][x] {
                    if let Cell::Empty = map[y + 1][x] {
                        map[y][x] = Cell::Empty;
                        map[y + 1][x] = Cell::RoundRock;
                        moved = true;
                    }
                }
            }
        }
    }
}
fn tilt_east(map: &mut Vec<Vec<Cell>>) {
    let width = map[0].len();
    let height = map.len();
    let mut moved = true;

    while moved {
        moved = false;

        for x in (0..width - 1).rev() {
            for y in 0..height {
                if let Cell::RoundRock = map[y][x] {
                    if let Cell::Empty = map[y][x + 1] {
                        map[y][x] = Cell::Empty;
                        map[y][x + 1] = Cell::RoundRock;
                        moved = true;
                    }
                }
            }
        }
    }
}

fn spin_cycle(map: &mut Vec<Vec<Cell>>) {
    tilt_north(map);
    tilt_west(map);
    tilt_south(map);
    tilt_east(map);
}

fn total_north_beam_load(map: &Vec<Vec<Cell>>) -> u32 {
    map.iter()
        .enumerate()
        .map(|(y, row)| {
            (row.iter().filter(|cell| **cell == Cell::RoundRock).count() * (map.len() - y)) as u32
        })
        .sum()
}

pub fn solve_part1(input: &str) -> u32 {
    let (_, mut map) = parse_map(input).unwrap();
    tilt_north(&mut map);
    total_north_beam_load(&map)
}

const CYCLES: usize = 1000000000;

pub fn solve_part2(input: &str) -> u32 {
    let (_, mut map) = parse_map(input).unwrap();

    let mut seen_maps = HashMap::from([(map.clone(), 0)]);

    for i in 0..CYCLES {
        spin_cycle(&mut map);

        if let Some(seen_i) = seen_maps.get(&map) {
            let cycle_length = i - seen_i;
            let remaining = CYCLES - i - 1;
            let remaining_i = remaining % cycle_length;

            for _ in 0..remaining_i {
                spin_cycle(&mut map);
            }
            break;
        }

        seen_maps.insert(map.clone(), i);
    }

    total_north_beam_load(&map)
}
