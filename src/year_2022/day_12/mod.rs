use std::collections::{HashMap, HashSet, VecDeque};

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
            assert_eq!(result, 31);
        }

        #[test]
        fn it_solves_input() {
            let result = solve_part1(INPUT);
            assert_eq!(result, 352);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn it_solves_example() {
            let result = solve_part2(EXAMPLE);
            assert_eq!(result, 29);
        }

        #[test]
        fn it_solves_input() {
            let result = solve_part2(INPUT);
            assert_eq!(result, 345);
        }
    }
}

type Map = Vec<Vec<char>>;
type Position = (usize, usize);

fn parse_map(input: &str) -> (Map, Position, Position) {
    let mut start = None;
    let mut end = None;
    let map = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| {
                    if c == 'S' {
                        start = Some((row, col));
                        'a'
                    } else if c == 'E' {
                        end = Some((row, col));
                        'z'
                    } else {
                        c
                    }
                })
                .collect::<Vec<char>>()
        })
        .collect::<Map>();

    (map, start.unwrap(), end.unwrap())
}

fn get_neighbors(map: &Map, (row, col): Position, flip: bool) -> Vec<Position> {
    let mut neighbors = Vec::new();

    let item = map[row][col];
    let mut neighbor_positions = vec![];

    if row > 0 {
        neighbor_positions.push((row - 1, col));
    }
    if row < map.len() - 1 {
        neighbor_positions.push((row + 1, col));
    }
    if col > 0 {
        neighbor_positions.push((row, col - 1));
    }
    if col < map[row].len() - 1 {
        neighbor_positions.push((row, col + 1));
    }

    for (row, col) in &neighbor_positions {
        let neighbor = map[*row][*col];
        let diff = if flip {
            item as i32 - neighbor as i32
        } else {
            neighbor as i32 - item as i32
        };
        if diff <= 1 {
            neighbors.push((*row, *col));
        }
    }

    neighbors
}

pub fn solve_part1(input: &str) -> u32 {
    let (map, start, end) = parse_map(input);
    let mut visited: HashSet<Position> = HashSet::new();
    let mut q = VecDeque::new();
    let mut prev: HashMap<Position, Position> = HashMap::new();

    visited.insert(start);
    q.push_back(start);

    while let Some(pos) = q.pop_front() {
        if pos == end {
            let mut step = 0;
            let mut curr = &end;

            while let Some(pos) = prev.get(curr) {
                step += 1;
                curr = pos;
            }
            return step;
        }

        let neighbors = get_neighbors(&map, pos, false);

        for neighbor in neighbors {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                prev.insert(neighbor, pos);
                q.push_back(neighbor);
            }
        }
    }

    unreachable!();
}

pub fn solve_part2(input: &str) -> u32 {
    let (map, _, start) = parse_map(input);
    let mut visited: HashSet<Position> = HashSet::new();
    let mut q = VecDeque::new();
    let mut prev: HashMap<Position, Position> = HashMap::new();

    visited.insert(start);
    q.push_back(start);

    while let Some(pos) = q.pop_front() {
        if map[pos.0][pos.1] == 'a' {
            let mut step = 0;
            let mut curr = &pos;

            while let Some(pos) = prev.get(curr) {
                step += 1;
                curr = pos;
            }
            return step;
        }

        let neighbors = get_neighbors(&map, pos, true);

        for neighbor in neighbors {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                prev.insert(neighbor, pos);
                q.push_back(neighbor);
            }
        }
    }

    unreachable!();
}
