use nom::{
    bytes::complete::tag,
    character::complete::{i32, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::{cmp::Ordering, collections::HashMap};

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
            assert_eq!(result, 24);
        }

        #[test]
        fn it_solves_input() {
            let result = solve_part1(INPUT);
            assert_eq!(result, 719);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn it_solves_example() {
            let result = solve_part2(EXAMPLE);
            assert_eq!(result, 93);
        }

        #[test]
        fn it_solves_input() {
            let result = solve_part2(INPUT);
            assert_eq!(result, 23390);
        }
    }
}

type Position = (i32, i32);

fn position(input: &str) -> IResult<&str, Position> {
    separated_pair(i32, tag(","), i32)(input)
}

fn position_sequence(input: &str) -> IResult<&str, Vec<Position>> {
    separated_list1(tag(" -> "), position)(input)
}

const SAND_SOURCE: Position = (500, 0);

struct Map {
    map: HashMap<Position, MapCell>,
    bounds: (Position, Position),
    floor: bool,
}

#[derive(Debug, PartialEq, Eq)]
enum MapCell {
    Rock,
    Sand,
    Air,
}

impl Map {
    fn parse(input: &str, floor: bool) -> Map {
        let (_, position_sequences) = separated_list1(newline, position_sequence)(input).unwrap();

        let mut map = HashMap::new();
        let mut bounds = (SAND_SOURCE, SAND_SOURCE);

        for row in position_sequences {
            for window in row.windows(2) {
                let (from, to) = (window[0], window[1]);

                let (to_x, to_y) = to;

                let mut curr = from;

                while curr != to {
                    map.insert(curr, MapCell::Rock);

                    let (curr_x, curr_y) = &mut curr;

                    bounds.0 = (bounds.0 .0.min(*curr_x), bounds.0 .1.min(*curr_y));
                    bounds.1 = (bounds.1 .0.max(*curr_x), bounds.1 .1.max(*curr_y));

                    match (to_x.cmp(curr_x), to_y.cmp(curr_y)) {
                        (Ordering::Less, Ordering::Equal) => *curr_x -= 1,
                        (Ordering::Greater, Ordering::Equal) => *curr_x += 1,
                        (Ordering::Equal, Ordering::Less) => *curr_y -= 1,
                        (Ordering::Equal, Ordering::Greater) => *curr_y += 1,
                        _ => panic!("We only support vertical / horizontal lines"),
                    }
                }

                map.insert(to, MapCell::Rock);
            }
        }

        Map { map, bounds, floor }
    }

    fn drop_sand(self: &mut Map) -> DropSandResult {
        let mut sand_pos = SAND_SOURCE;

        let mut changed = true;

        while changed {
            changed = false;
            let drop_options = [
                (sand_pos.0, sand_pos.1 + 1),
                (sand_pos.0 - 1, sand_pos.1 + 1),
                (sand_pos.0 + 1, sand_pos.1 + 1),
            ];

            for option in drop_options.iter() {
                let target = *option;

                let dest_cell = self.map.get(&target).unwrap_or(
                    if self.floor && target.1 >= self.bounds.1 .1 + 2 {
                        &MapCell::Rock
                    } else {
                        &MapCell::Air
                    },
                );

                if dest_cell == &MapCell::Air {
                    sand_pos = target;
                    changed = true;
                    break;
                }
            }

            // If we consider the floor, and sand_pos is equal to SAND_SOURCE, we have a blocked source
            if self.floor && sand_pos == SAND_SOURCE {
                return DropSandResult::BlockedSource;
            }

            // If sand_pos is out of self.bounds, we fell into the void
            if !self.floor
                && (sand_pos.0 < self.bounds.0 .0
                    || sand_pos.0 > self.bounds.1 .0
                    || sand_pos.1 < self.bounds.0 .1
                    || sand_pos.1 > self.bounds.1 .1)
            {
                return DropSandResult::FellIntoEndlessVoid;
            }
        }

        self.map.insert(sand_pos, MapCell::Sand);

        DropSandResult::CameToRest
    }
}

#[derive(Debug)]
enum DropSandResult {
    CameToRest,
    FellIntoEndlessVoid,
    BlockedSource,
}

pub fn solve_part1(input: &str) -> u32 {
    let mut map = Map::parse(input, false);

    let mut count = 0;

    while let DropSandResult::CameToRest = map.drop_sand() {
        count += 1;
    }

    count
}

pub fn solve_part2(input: &str) -> u32 {
    let mut map = Map::parse(input, true);

    let mut count = 1;

    while let DropSandResult::CameToRest = map.drop_sand() {
        count += 1;
    }

    count
}
