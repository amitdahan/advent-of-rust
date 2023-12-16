use itertools::Itertools;
use rayon::prelude::*;
use std::collections::{HashSet, VecDeque};

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.in");

    mod part1 {
        use super::*;

        #[test]
        fn it_solves_example() {
            let result = solve_part1(EXAMPLE);
            assert_eq!(result, 46);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn it_solves_example() {
            let result = solve_part2(EXAMPLE);
            assert_eq!(result, 51);
        }
    }
}

fn parse_map(input: &str) -> Vec<Vec<char>> {
    input.trim().lines().map(|l| l.chars().collect()).collect()
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn count_energized(map: &[Vec<char>], start: (Direction, usize, usize)) -> usize {
    let height = map.len();
    let width = map[0].len();

    let mut visited = HashSet::new();
    let mut q = VecDeque::from([start]);

    while let Some(curr) = q.pop_front() {
        if !visited.insert(curr) {
            continue;
        }

        use Direction::*;

        let curr = {
            let (dir, x, y) = curr;
            (dir, map[y][x], x, y)
        };

        match curr {
            (Up, '.', _, 0) | (Up, '|', _, 0) => (),
            (Up, '.', x, y) | (Up, '|', x, y) => q.push_back((Up, x, y - 1)),
            (Up, '/', x, _) if x >= width - 1 => (),
            (Up, '/', x, y) => q.push_back((Right, x + 1, y)),
            (Up, '\\', 0, _) => (),
            (Up, '\\', x, y) => q.push_back((Left, x - 1, y)),
            (Up, '-', 0, y) => q.push_back((Right, 1, y)),
            (Up, '-', x, y) if x == width - 1 => q.push_back((Left, x - 1, y)),
            (Up, '-', x, y) => {
                q.push_back((Left, x - 1, y));
                q.push_back((Right, x + 1, y));
            }
            (Right, '.', x, _) | (Right, '-', x, _) if x >= width - 1 => (),
            (Right, '.', x, y) | (Right, '-', x, y) => q.push_back((Right, x + 1, y)),
            (Right, '/', _, 0) => (),
            (Right, '/', x, y) => q.push_back((Up, x, y - 1)),
            (Right, '\\', _, y) if y >= height - 1 => (),
            (Right, '\\', x, y) => q.push_back((Down, x, y + 1)),
            (Right, '|', x, 0) => q.push_back((Down, x, 1)),
            (Right, '|', x, y) if y == height - 1 => q.push_back((Up, x, y - 1)),
            (Right, '|', x, y) => {
                q.push_back((Up, x, y - 1));
                q.push_back((Down, x, y + 1));
            }
            (Down, '.', _, y) | (Down, '|', _, y) if y >= height - 1 => (),
            (Down, '.', x, y) | (Down, '|', x, y) => q.push_back((Down, x, y + 1)),
            (Down, '/', 0, _) => (),
            (Down, '/', x, y) => q.push_back((Left, x - 1, y)),
            (Down, '\\', x, _) if x >= width - 1 => (),
            (Down, '\\', x, y) => q.push_back((Right, x + 1, y)),
            (Down, '-', 0, y) => q.push_back((Right, 1, y)),
            (Down, '-', x, y) if x == width - 1 => q.push_back((Left, x - 1, y)),
            (Down, '-', x, y) => {
                q.push_back((Left, x - 1, y));
                q.push_back((Right, x + 1, y));
            }
            (Left, '.', 0, _) | (Left, '-', 0, _) => (),
            (Left, '.', x, y) | (Left, '-', x, y) => q.push_back((Left, x - 1, y)),
            (Left, '/', _, y) if y >= height - 1 => (),
            (Left, '/', x, y) => q.push_back((Down, x, y + 1)),
            (Left, '\\', _, 0) => (),
            (Left, '\\', x, y) => q.push_back((Up, x, y - 1)),
            (Left, '|', x, 0) => q.push_back((Down, x, 1)),
            (Left, '|', x, y) if y == height - 1 => q.push_back((Up, x, y - 1)),
            (Left, '|', x, y) => {
                q.push_back((Up, x, y - 1));
                q.push_back((Down, x, y + 1));
            }
            _ => unreachable!(),
        }
    }

    visited.iter().map(|(_, x, y)| (x, y)).unique().count()
}

pub fn solve_part1(input: &str) -> usize {
    let map = parse_map(input);
    count_energized(&map, (Direction::Right, 0usize, 0usize))
}

pub fn solve_part2(input: &str) -> usize {
    let map = parse_map(input);

    let height = map.len();
    let width = map[0].len();

    (0..width)
        .flat_map(|x| [(Direction::Down, x, 0), (Direction::Up, x, height - 1)])
        .chain(
            (0..height).flat_map(|y| [(Direction::Right, 0, y), (Direction::Left, width - 1, y)]),
        )
        .par_bridge()
        .map(|start| count_energized(&map, start))
        .max()
        .unwrap()
}
