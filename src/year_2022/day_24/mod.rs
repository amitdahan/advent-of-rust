use std::collections::{HashMap, HashSet, VecDeque};

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.in");
    const INPUT: &str = include_str!("input.in");

    #[test]
    fn it_finds_initial_position_of_blizzard() {
        let width = 4;
        let height = 4;

        use Direction::*;

        assert_eq!(rewind((0, 0), Up, width, height, 1), (0, 1));
        assert_eq!(rewind((0, 0), Up, width, height, 2), (0, 2));
        assert_eq!(rewind((0, 0), Up, width, height, 0), (0, 0));
        assert_eq!(rewind((0, 0), Up, width, height, height), (0, 0));
        assert_eq!(rewind((0, 0), Down, width, height, 1), (0, 3));
        assert_eq!(rewind((0, 0), Down, width, height, 2), (0, 2));
        assert_eq!(rewind((0, 0), Down, width, height, 0), (0, 0));
        assert_eq!(rewind((0, 0), Down, width, height, height), (0, 0));
        assert_eq!(rewind((0, 0), Left, width, height, 1), (1, 0));
        assert_eq!(rewind((0, 0), Left, width, height, 2), (2, 0));
        assert_eq!(rewind((0, 0), Left, width, height, 0), (0, 0));
        assert_eq!(rewind((0, 0), Left, width, height, width), (0, 0));
        assert_eq!(rewind((0, 0), Right, width, height, 1), (3, 0));
        assert_eq!(rewind((0, 0), Right, width, height, 2), (2, 0));
        assert_eq!(rewind((0, 0), Right, width, height, 0), (0, 0));
        assert_eq!(rewind((0, 0), Right, width, height, width), (0, 0));
    }

    mod part1 {
        use super::*;

        #[test]
        fn it_solves_example() {
            let result = solve_part1(EXAMPLE);
            assert_eq!(result, 18);
        }

        #[test]
        fn it_solves_input() {
            let result = solve_part1(INPUT);
            assert_eq!(result, 279);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn it_solves_example() {
            let result = solve_part2(EXAMPLE);
            assert_eq!(result, 54);
        }

        #[test]
        fn it_solves_input() {
            let result = solve_part2(INPUT);
            assert_eq!(result, 762);
        }
    }
}

type Position = (i32, i32);

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn reversed(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

fn parse(input: &str) -> (u32, u32, HashMap<Position, Direction>) {
    let mut blizzards: HashMap<Position, Direction> = HashMap::new();

    let mut width = 0;
    let mut height = 0;

    let lines = input.lines().collect::<Vec<_>>();

    for (y, line) in lines[1..lines.len() - 1].iter().enumerate() {
        height = height.max(y as u32 + 1);

        let chars = line.chars().collect::<Vec<_>>();

        for (x, c) in chars[1..chars.len() - 1].iter().enumerate() {
            width = width.max(x as u32 + 1);

            let pos = (x as i32, y as i32);

            let dir = match c {
                '^' => Direction::Up,
                'v' => Direction::Down,
                '<' => Direction::Left,
                '>' => Direction::Right,
                _ => continue,
            };

            blizzards.insert(pos, dir);
        }
    }

    (width, height, blizzards)
}

fn rewind((x, y): Position, dir: Direction, width: u32, height: u32, time: u32) -> Position {
    let reversed = dir.reversed();
    let (dx, dy) = (
        match reversed {
            Direction::Left => -1,
            Direction::Right => 1,
            _ => 0,
        },
        match reversed {
            Direction::Up => -1,
            Direction::Down => 1,
            _ => 0,
        },
    );

    let (x, y) = (
        (x + dx as i32 * time as i32) % width as i32,
        (y + dy as i32 * time as i32) % height as i32,
    );

    (
        if x < 0 { x + width as i32 } else { x },
        if y < 0 { y + height as i32 } else { y },
    )
}

fn neighbors(pos: Position, width: u32, height: u32, end: Position) -> Vec<Position> {
    let (x, y) = pos;

    let mut result = Vec::new();

    for (dx, dy) in &[(0, -1), (0, 1), (-1, 0), (1, 0)] {
        let (x, y) = (x + dx, y + dy);

        if x >= 0 && y >= 0 && x < width as i32 && y < height as i32 {
            result.push((x, y));
        } else if (x, y) == end {
            result.push((x, y));
        }
    }

    result
}

fn can_move(
    blizzards: &HashMap<Position, Direction>,
    pos: Position,
    time: u32,
    width: u32,
    height: u32,
    start: Position,
    end: Position,
) -> bool {
    use Direction::*;

    let possible_up_blizzard = blizzards
        .get(&rewind(pos, Up, width, height, time))
        .filter(|x| **x == Up);
    let possible_down_blizzard = blizzards
        .get(&rewind(pos, Down, width, height, time))
        .filter(|x| **x == Down);
    let possible_left_blizzard = blizzards
        .get(&rewind(pos, Left, width, height, time))
        .filter(|x| **x == Left);
    let possible_right_blizzard = blizzards
        .get(&rewind(pos, Right, width, height, time))
        .filter(|x| **x == Right);

    pos == end
        || pos == start
        || [
            possible_up_blizzard,
            possible_down_blizzard,
            possible_left_blizzard,
            possible_right_blizzard,
        ]
        .iter()
        .all(|o| o.is_none())
}

fn get_best_time(
    blizzards: &HashMap<Position, Direction>,
    width: u32,
    height: u32,
    start: Position,
    end: Position,
    start_time: u32,
) -> u32 {
    let mut visited: HashSet<(Position, u32)> = HashSet::from_iter([(start, start_time)]);
    let mut q: VecDeque<(Position, u32)> = VecDeque::new();
    q.push_front((start, start_time));

    while let Some((pos, time)) = q.pop_front() {
        if pos == end {
            return time;
        }
        let next_time = time + 1;
        for next_pos in neighbors(pos, width, height, end) {
            if visited.contains(&(next_pos, next_time)) {
                continue;
            }

            if can_move(&blizzards, next_pos, next_time, width, height, start, end) {
                visited.insert((next_pos, next_time));
                q.push_back((next_pos, next_time));
            }
        }
        if can_move(&blizzards, pos, next_time, width, height, start, end) {
            visited.insert((pos, next_time));
            q.push_back((pos, next_time));
        }
    }

    unreachable!();
}

pub fn solve_part1(input: &str) -> u32 {
    let (width, height, blizzards) = parse(input);

    let start = (0, -1);
    let end = (width as i32 - 1, height as i32);

    get_best_time(&blizzards, width, height, start, end, 0)
}

pub fn solve_part2(input: &str) -> u32 {
    let (width, height, blizzards) = parse(input);

    let start = (0, -1);
    let end = (width as i32 - 1, height as i32);

    let time_to_reach_goal = get_best_time(&blizzards, width, height, start, end, 0);
    let time_to_get_back_for_snacks =
        get_best_time(&blizzards, width, height, end, start, time_to_reach_goal);
    get_best_time(
        &blizzards,
        width,
        height,
        start,
        end,
        time_to_get_back_for_snacks,
    )
}
