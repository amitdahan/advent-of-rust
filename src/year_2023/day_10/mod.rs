use std::collections::{HashSet, VecDeque};

use num::Integer;

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = include_str!("example_1.in");
    const EXAMPLE_2: &str = include_str!("example_2.in");
    const EXAMPLE_3: &str = include_str!("example_3.in");
    const EXAMPLE_4: &str = include_str!("example_4.in");
    const EXAMPLE_5: &str = include_str!("example_5.in");
    const EXAMPLE_6: &str = include_str!("example_6.in");

    mod part1 {
        use super::*;

        #[test]
        fn it_solves_example_1() {
            let result = solve_part1(EXAMPLE_1);
            assert_eq!(result, 4);
        }

        #[test]
        fn it_solves_example_2() {
            let result = solve_part1(EXAMPLE_2);
            assert_eq!(result, 8);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn it_solves_example_3() {
            let result = solve_part2(EXAMPLE_3);
            assert_eq!(result, 4);
        }

        #[test]
        fn it_solves_example_4() {
            let result = solve_part2(EXAMPLE_4);
            assert_eq!(result, 4);
        }

        #[test]
        fn it_solves_example_5() {
            let result = solve_part2(EXAMPLE_5);
            assert_eq!(result, 8);
        }

        #[test]
        fn it_solves_example_6() {
            let result = solve_part2(EXAMPLE_6);
            assert_eq!(result, 10);
        }
    }
}

fn parse(input: &str) -> (Vec<Vec<char>>, (usize, usize)) {
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let start = map
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .enumerate()
                .find_map(|(x, &c)| if c == 'S' { Some((x, y)) } else { None })
        })
        .unwrap();

    (map, start)
}

fn neighbors(map: &[Vec<char>], &(x, y): &(usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();

    // North
    if y > 0
        && match (map[y][x], map[y - 1][x]) {
            (_, '.') => false,
            (_, 'S') => true,
            ('-', _) => false,
            ('7', _) => false,
            ('F', _) => false,
            (_, '|') => true,
            (_, '7') => true,
            (_, 'F') => true,
            _ => false,
        }
    {
        neighbors.push((x, y - 1));
    }

    // East
    if let Some(neighbor) = map.get(y).and_then(|row| row.get(x + 1)) {
        if match (map[y][x], neighbor) {
            (_, '.') => false,
            (_, 'S') => true,
            ('|', _) => false,
            ('7', _) => false,
            ('J', _) => false,
            (_, '-') => true,
            (_, '7') => true,
            (_, 'J') => true,
            _ => false,
        } {
            neighbors.push((x + 1, y));
        }
    }

    // South
    if let Some(neighbor) = map.get(y + 1).and_then(|row| row.get(x)) {
        if match (map[y][x], neighbor) {
            (_, '.') => false,
            (_, 'S') => true,
            ('-', _) => false,
            ('L', _) => false,
            ('J', _) => false,
            (_, '|') => true,
            (_, 'L') => true,
            (_, 'J') => true,
            _ => false,
        } {
            neighbors.push((x, y + 1));
        }
    }

    // West
    if x > 0
        && match (map[y][x], map[y][x - 1]) {
            (_, '.') => false,
            (_, 'S') => true,
            ('|', _) => false,
            ('L', _) => false,
            ('F', _) => false,
            (_, '-') => true,
            (_, 'L') => true,
            (_, 'F') => true,
            _ => false,
        }
    {
        neighbors.push((x - 1, y));
    }

    neighbors
}

pub fn solve_part1(input: &str) -> u32 {
    let (map, start) = parse(input);

    let mut visited = HashSet::new();
    let mut queue = VecDeque::from([start]);
    let mut level = 0;

    while !queue.is_empty() {
        for _ in 0..queue.len() {
            let curr = queue.pop_front().unwrap();
            if visited.contains(&curr) {
                continue;
            }
            visited.insert(curr);

            for neighbor in neighbors(&map, &curr) {
                queue.push_back(neighbor);
            }
        }
        level += 1;
    }

    level - 2
}

pub fn solve_part2(input: &str) -> u32 {
    let (map, start) = parse(input);

    let mut visited = HashSet::new();
    let mut queue = VecDeque::from([start]);

    while let Some(curr) = queue.pop_front() {
        if visited.contains(&curr) {
            continue;
        }
        visited.insert(curr);

        for neighbor in neighbors(&map, &curr) {
            queue.push_back(neighbor);
        }
    }

    let mut count = 0;

    for y in 0..map.len() {
        let mut intersects = 0;

        for x in 0..map[y].len() {
            if visited.contains(&(x, y)) {
                if matches!(map[y][x], 'J' | 'L' | '|') {
                    intersects += 1;
                }
            } else if intersects.is_odd() {
                count += 1;
            }
        }
    }

    count
}
