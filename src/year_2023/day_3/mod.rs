use std::collections::HashSet;

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
            assert_eq!(result, 4361);
        }

        #[test]
        fn it_solves_input() {
            let result = solve_part1(INPUT);
            assert_eq!(result, 0);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn it_solves_example() {
            let result = solve_part2(EXAMPLE);
            assert_eq!(result, 467835);
        }

        #[test]
        fn it_solves_input() {
            let result = solve_part2(INPUT);
            assert_eq!(result, 0);
        }
    }
}

#[derive(Debug, PartialEq)]
enum Cell {
    Digit(u8),
    Empty,
    Symbol(char),
}

fn parse_grid(input: &str) -> Box<[Box<[Cell]>]> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '0'..='9' => Cell::Digit(c.to_digit(10).unwrap() as u8),
                    '.' => Cell::Empty,
                    _ => Cell::Symbol(c),
                })
                .collect::<Vec<_>>()
                .into_boxed_slice()
        })
        .collect::<Vec<_>>()
        .into_boxed_slice()
}

fn has_adjacent_symbol(grid: &[Box<[Cell]>], x: usize, y: usize) -> bool {
    let width = grid[0].len();
    let height = grid.len();

    let mut has_adjacent_symbol = false;

    for y_offset in -1..=1 {
        for x_offset in -1..=1 {
            if x_offset == 0 && y_offset == 0 {
                continue;
            }

            let x = x as i32 + x_offset;
            let y = y as i32 + y_offset;

            if x < 0 || x >= width as i32 || y < 0 || y >= height as i32 {
                continue;
            }

            if let Cell::Symbol(_) = grid[y as usize][x as usize] {
                has_adjacent_symbol = true;
                break;
            }
        }
    }

    has_adjacent_symbol
}

pub fn solve_part1(input: &str) -> u32 {
    let grid = parse_grid(input);
    let width = grid[0].len();
    let height = grid.len();

    let mut sum = 0;

    for y in 0..height {
        let mut curr_num = None;
        let mut adj_symbol = false;

        for x in 0..width {
            match grid[y][x] {
                Cell::Digit(d) => {
                    adj_symbol = adj_symbol || has_adjacent_symbol(&grid, x, y);
                    curr_num = match curr_num {
                        Some(n) => Some(n * 10 + d as u32),
                        None => Some(d as u32),
                    };
                }
                _ => {
                    if let Some(n) = curr_num {
                        if adj_symbol {
                            sum += n;
                        }
                    }

                    curr_num = None;
                    adj_symbol = false;
                }
            }
        }

        if let Some(n) = curr_num {
            if adj_symbol {
                sum += n;
            }
        }
    }

    sum
}

fn get_adjacent_numbers(grid: &[Box<[Cell]>], x: usize, y: usize) -> Vec<u32> {
    let width = grid[0].len();
    let height = grid.len();

    let mut adj_nums = Vec::new();
    let mut seen = HashSet::new();

    for y_offset in -1..=1 {
        for x_offset in -1..=1 {
            if x_offset == 0 && y_offset == 0 {
                continue;
            }

            let x = x as i32 + x_offset;
            let y = y as i32 + y_offset;

            if x < 0 || x >= width as i32 || y < 0 || y >= height as i32 {
                continue;
            }

            if let Cell::Digit(_) = grid[y as usize][x as usize] {
                let mut x = x;

                while x > 0 && matches!(grid[y as usize][(x - 1) as usize], Cell::Digit(_)) {
                    x -= 1;
                }

                if !seen.insert((x, y)) {
                    continue;
                }

                let mut num = if let Cell::Digit(d) = grid[y as usize][x as usize] {
                    d as u32
                } else {
                    unreachable!()
                };

                while x + 1 < width as i32
                    && matches!(grid[y as usize][(x + 1) as usize], Cell::Digit(_))
                {
                    x += 1;

                    if let Cell::Digit(d) = grid[y as usize][x as usize] {
                        num = num * 10 + d as u32;
                    } else {
                        unreachable!()
                    }
                }

                adj_nums.push(num);
            }
        }
    }

    adj_nums
}

pub fn solve_part2(input: &str) -> u32 {
    let grid = parse_grid(input);
    let width = grid[0].len();
    let height = grid.len();

    let mut sum = 0;

    for y in 0..height {
        for x in 0..width {
            if let Cell::Symbol('*') = grid[y][x] {
                let adj_nums = get_adjacent_numbers(&grid, x, y);

                if adj_nums.len() == 2 {
                    sum += adj_nums[0] * adj_nums[1];
                }
            }
        }
    }

    sum
}
