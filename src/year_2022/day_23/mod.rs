use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.in");
    const INPUT: &str = include_str!("input.in");

    #[test]
    fn it_parses() {
        let data = parse(EXAMPLE);
        assert_eq!(
            data,
            HashSet::from_iter([
                (5, 5),
                (4, 1),
                (1, 6),
                (6, 2),
                (6, 5),
                (4, 0),
                (5, 3),
                (0, 5),
                (3, 1),
                (0, 2),
                (0, 4),
                (6, 3),
                (1, 5),
                (4, 6),
                (4, 4),
                (1, 3),
                (6, 1),
                (2, 1),
                (3, 4),
                (4, 2),
                (2, 4),
                (3, 5)
            ])
        );
    }

    mod part1 {
        use super::*;

        #[test]
        fn it_solves_example() {
            let result = solve_part1(EXAMPLE);
            assert_eq!(result, 110);
        }

        #[test]
        fn it_solves_input() {
            let result = solve_part1(INPUT);
            assert_eq!(result, 3_862);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn it_solves_example() {
            let result = solve_part2(EXAMPLE);
            assert_eq!(result, 20);
        }

        #[test]
        fn it_solves_input() {
            let result = solve_part2(INPUT);
            assert_eq!(result, 913);
        }
    }
}

type Position = (i32, i32);

fn parse(input: &str) -> HashSet<Position> {
    let mut map = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                map.insert((x as i32, y as i32));
            }
        }
    }

    map
}

const ALL_NEIGHBORS: [Position; 8] = [
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

const INITIAL_PRIORITIES: [[Position; 3]; 4] = [
    [(0, -1), (1, -1), (-1, -1)],
    [(0, 1), (1, 1), (-1, 1)],
    [(-1, 0), (-1, -1), (-1, 1)],
    [(1, 0), (1, -1), (1, 1)],
];

fn step(map: &mut HashSet<Position>, priorities: &VecDeque<[(i32, i32); 3]>) -> bool {
    let mut moves = HashMap::<Position, Position>::new();

    for &(x, y) in map.iter() {
        if ALL_NEIGHBORS
            .iter()
            .map(|(dx, dy)| (x + dx, y + dy))
            .any(|(x, y)| map.contains(&(x, y)))
        {
            if let Some(priority) = priorities.iter().find(|priority| {
                priority
                    .iter()
                    .map(|(dx, dy)| (x + dx, y + dy))
                    .all(|(x, y)| !map.contains(&(x, y)))
            }) {
                moves.insert(
                    (x, y),
                    priority.first().map(|(dx, dy)| (x + dx, y + dy)).unwrap(),
                );
            }
        }
    }

    let mut destinations = HashMap::new();
    for dest in moves.values() {
        *destinations.entry(*dest).or_insert(0) += 1;
    }

    moves.retain(|_, dest| destinations[dest] == 1);

    for (src, dest) in &moves {
        map.remove(src);
        map.insert(*dest);
    }

    !moves.is_empty()
}

pub fn solve_part1(input: &str) -> u32 {
    let mut map = parse(input);
    let mut priorities = VecDeque::from_iter(INITIAL_PRIORITIES);

    for _ in 0..10 {
        step(&mut map, &priorities);
        priorities.rotate_left(1);
    }

    let (min_x, max_x) = map.iter().map(|&(x, _)| x).minmax().into_option().unwrap();
    let (min_y, max_y) = map.iter().map(|&(_, y)| y).minmax().into_option().unwrap();
    let width = (max_x - min_x + 1) as u32;
    let height = (max_y - min_y + 1) as u32;

    width * height - map.len() as u32
}

pub fn solve_part2(input: &str) -> u32 {
    let mut map = parse(input);
    let mut priorities = VecDeque::from_iter(INITIAL_PRIORITIES);
    let mut round = 0;

    loop {
        round += 1;
        if !step(&mut map, &priorities) {
            break;
        }
        priorities.rotate_left(1);
    }

    round
}
