use std::collections::HashSet;

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
            assert_eq!(result, 374);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn it_solves_example_1() {
            let result = solve_part2(EXAMPLE, 10);
            assert_eq!(result, 1030);
        }

        #[test]
        fn it_solves_example_2() {
            let result = solve_part2(EXAMPLE, 100);
            assert_eq!(result, 8410);
        }
    }
}

type Map = HashSet<(usize, usize)>;

fn parse_map(input: &str) -> Map {
    let mut map = Map::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                map.insert((x, y));
            }
        }
    }
    map
}

fn expand_cosmos(map: &mut Map, age: &usize) {
    let width = map.iter().map(|(x, _)| x).max().unwrap() + 1;
    let height = map.iter().map(|(_, y)| y).max().unwrap() + 1;

    for y in (0..height).rev() {
        if map.iter().any(|(_, y2)| y2 == &y) {
            continue;
        }

        let affected = map
            .iter()
            .filter(|(_, y2)| &y < y2)
            .cloned()
            .collect::<Vec<_>>();

        for coords in affected {
            map.remove(&coords);
            map.insert((coords.0, coords.1 + age - 1));
        }
    }

    for x in (0..width).rev() {
        if map.iter().any(|(x2, _)| x2 == &x) {
            continue;
        }

        let affected = map
            .iter()
            .filter(|(x2, _)| &x < x2)
            .cloned()
            .collect::<Vec<_>>();

        for coords in affected {
            map.remove(&coords);
            map.insert((coords.0 + age - 1, coords.1));
        }
    }
}

fn distance((a, b): (&(usize, usize), &(usize, usize))) -> u64 {
    ((a.0 as i64 - b.0 as i64).abs() + (a.1 as i64 - b.1 as i64).abs()) as u64
}
fn sum_shortest_paths(map: &Map) -> u64 {
    map.iter().tuple_combinations().map(distance).sum()
}

pub fn solve_part1(input: &str) -> u64 {
    let mut map = parse_map(input);
    expand_cosmos(&mut map, &2);
    sum_shortest_paths(&map)
}
pub fn solve_part2(input: &str, age: usize) -> u64 {
    let mut map = parse_map(input);
    expand_cosmos(&mut map, &age);
    sum_shortest_paths(&map)
}
