use std::collections::{HashSet, VecDeque};

use itertools::Itertools;
use polyfit_rs::polyfit_rs::polyfit;

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.in");

    mod part1 {
        use super::*;

        #[test]
        fn it_solves_example() {
            let result = solve_part1(EXAMPLE, 6);
            assert_eq!(result, 16);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn it_solves_example_6_steps() {
            let (map, start) = parse(EXAMPLE);
            let result = reachable_plots(&map, start, 6, true);
            assert_eq!(result, 16);
        }

        #[test]
        fn it_solves_example_10_steps() {
            let (map, start) = parse(EXAMPLE);
            let result = reachable_plots(&map, start, 10, true);
            assert_eq!(result, 50);
        }

        #[test]
        fn it_solves_example_50_steps() {
            let (map, start) = parse(EXAMPLE);
            let result = reachable_plots(&map, start, 50, true);
            assert_eq!(result, 1594);
        }

        #[test]
        fn it_solves_example_100_steps() {
            let (map, start) = parse(EXAMPLE);
            let result = reachable_plots(&map, start, 100, true);
            assert_eq!(result, 6536);
        }

        #[test]
        #[ignore]
        fn it_solves_example_500_steps() {
            let (map, start) = parse(EXAMPLE);
            let result = reachable_plots(&map, start, 500, true);
            assert_eq!(result, 167004);
        }

        #[test]
        #[ignore]
        fn it_solves_example_1000_steps() {
            let (map, start) = parse(EXAMPLE);
            let result = reachable_plots(&map, start, 1000, true);
            assert_eq!(result, 668697);
        }

        #[test]
        #[ignore]
        fn it_solves_example_5000_steps() {
            let (map, start) = parse(EXAMPLE);
            let result = reachable_plots(&map, start, 5000, true);
            assert_eq!(result, 16733044);
        }
    }
}

fn parse(input: &str) -> (Vec<Vec<bool>>, (i32, i32)) {
    let start = input
        .lines()
        .enumerate()
        .find_map(|(y, line)| {
            line.chars().enumerate().find_map(|(x, c)| match c {
                'S' => Some((x as i32, y as i32)),
                _ => None,
            })
        })
        .unwrap();

    let map = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => true,
                    '.' | 'S' => false,
                    _ => panic!("Invalid input"),
                })
                .collect()
        })
        .collect::<Vec<_>>();

    (map, start)
}

fn reachable_plots(
    map: &[Vec<bool>],
    start: (i32, i32),
    target_steps: usize,
    infinite: bool,
) -> usize {
    let width = map[0].len();
    let height = map.len();

    let mut visited = HashSet::new();
    let mut q = VecDeque::from([(start, 0)]);

    while let Some(((x, y), steps)) = q.pop_front() {
        if !visited.insert(((x, y), steps)) {
            continue;
        }

        if steps < target_steps {
            if infinite {
                let wrapped_x = x.rem_euclid(width as i32);
                let wrapped_y = y.rem_euclid(height as i32);

                if !map[wrapped_y as usize][(x - 1).rem_euclid(width as i32) as usize] {
                    q.push_back(((x - 1, y), steps + 1));
                }
                if !map[(y - 1).rem_euclid(height as i32) as usize][wrapped_x as usize] {
                    q.push_back(((x, y - 1), steps + 1));
                }
                if !map[wrapped_y as usize][(x + 1).rem_euclid(width as i32) as usize] {
                    q.push_back(((x + 1, y), steps + 1));
                }
                if !map[(y + 1).rem_euclid(height as i32) as usize][wrapped_x as usize] {
                    q.push_back(((x, y + 1), steps + 1));
                }
            } else {
                if x > 0 && !map[y as usize][x as usize - 1] {
                    q.push_back(((x - 1, y), steps + 1));
                }
                if y > 0 && !map[y as usize - 1][x as usize] {
                    q.push_back(((x, y - 1), steps + 1));
                }
                if (x as usize) < width - 1 && !map[y as usize][x as usize + 1] {
                    q.push_back(((x + 1, y), steps + 1));
                }
                if (y as usize) < height - 1 && !map[y as usize + 1][x as usize] {
                    q.push_back(((x, y + 1), steps + 1));
                }
            }
        }
    }

    visited
        .iter()
        .filter(|(_, steps)| steps == &target_steps)
        .map(|((x, y), _)| (x, y))
        .unique()
        .count()
}

pub fn solve_part1(input: &str, steps: usize) -> usize {
    let (map, start) = parse(input);
    reachable_plots(&map, start, steps, false)
}

pub fn solve_part2(input: &str) -> usize {
    let (map, start) = parse(input);
    let size = map.len();
    let x0 = reachable_plots(&map, start, size / 2 + size * 0, true);
    let x1 = reachable_plots(&map, start, size / 2 + size * 1, true);
    let x2 = reachable_plots(&map, start, size / 2 + size * 2, true);
    let steps = 26501365;
    let xn = ((steps - size / 2) / size) as i64;
    let result = polyfit(&[0.0, 1.0, 2.0], &[x0 as f32, x1 as f32, x2 as f32], 2).unwrap();
    let (a, b, c) = (
        result[2].round() as i64,
        result[1].round() as i64,
        result[0].round() as i64,
    );
    (a * xn.pow(2) + b * xn + c) as usize
}
