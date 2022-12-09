use std::collections::HashSet;

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = include_str!("example_1.in");
    const EXAMPLE_2: &str = include_str!("example_2.in");
    const INPUT: &str = include_str!("input.in");

    mod part1 {
        use super::*;

        #[test]
        fn it_solves_example() {
            let result = solve_part1(EXAMPLE_1);
            assert_eq!(result, 13);
        }

        #[test]
        fn it_solves_input() {
            let result = solve_part1(INPUT);
            assert_eq!(result, 6337);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn it_solves_example_1() {
            let result = solve_part2(EXAMPLE_1);
            assert_eq!(result, 1);
        }

        #[test]
        fn it_solves_example_2() {
            let result = solve_part2(EXAMPLE_2);
            assert_eq!(result, 36);
        }

        #[test]
        fn it_solves_input() {
            let result = solve_part2(INPUT);
            assert_eq!(result, 2455);
        }
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn go(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => self.y += 1,
            Direction::Right => self.x += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
        };
    }

    fn follow(&mut self, other: &Position) {
        let Position {
            x: other_x,
            y: other_y,
        } = other;

        let Position {
            x: self_x,
            y: self_y,
        } = self;

        let (move_x, move_y) = match (*other_x - *self_x, *other_y - *self_y) {
            (0, y) if y > 1 => (0, 1),
            (0, y) if y < -1 => (0, -1),
            (x, 0) if x > 1 => (1, 0),
            (x, 0) if x < -1 => (-1, 0),
            (x, y) if x >= 1 && y > 1 => (1, 1),
            (x, y) if x > 1 && y >= 1 => (1, 1),
            (x, y) if x >= 1 && y < -1 => (1, -1),
            (x, y) if x > 1 && y <= -1 => (1, -1),
            (x, y) if x <= -1 && y > 1 => (-1, 1),
            (x, y) if x < -1 && y >= 1 => (-1, 1),
            (x, y) if x <= -1 && y < -1 => (-1, -1),
            (x, y) if x < -1 && y <= -1 => (-1, -1),
            _ => (0, 0),
        };

        self.x += move_x;
        self.y += move_y;
    }
}

fn parse_moves(input: &str) -> Vec<(Direction, usize)> {
    input
        .lines()
        .map(|line| {
            let (direction, distance) = line.split_once(' ').unwrap();
            let direction = match direction {
                "U" => Direction::Up,
                "R" => Direction::Right,
                "D" => Direction::Down,
                "L" => Direction::Left,
                _ => panic!("Invalid direction"),
            };
            let distance = distance.parse::<usize>().unwrap();
            (direction, distance)
        })
        .collect()
}

pub fn solve_part1(input: &str) -> i32 {
    let head_moves: Vec<(Direction, usize)> = parse_moves(input);
    let mut tail_visited: HashSet<Position> = HashSet::new();

    let mut head = Position { x: 0, y: 0 };
    let mut tail = Position { x: 0, y: 0 };

    for (direction, distance) in &head_moves {
        for _ in 0..*distance {
            head.go(direction);
            tail.follow(&head);
            tail_visited.insert(tail.clone());
        }
    }

    tail_visited.len() as i32
}

pub fn solve_part2(input: &str) -> i32 {
    let head_moves: Vec<(Direction, usize)> = parse_moves(input);
    let mut tail_visited: HashSet<Position> = HashSet::new();

    let mut rope = (
        Position { x: 0, y: 0 },
        Position { x: 0, y: 0 },
        Position { x: 0, y: 0 },
        Position { x: 0, y: 0 },
        Position { x: 0, y: 0 },
        Position { x: 0, y: 0 },
        Position { x: 0, y: 0 },
        Position { x: 0, y: 0 },
        Position { x: 0, y: 0 },
        Position { x: 0, y: 0 },
    );

    for (direction, distance) in &head_moves {
        for _ in 0..*distance {
            rope.0.go(direction);
            rope.1.follow(&rope.0);
            rope.2.follow(&rope.1);
            rope.3.follow(&rope.2);
            rope.4.follow(&rope.3);
            rope.5.follow(&rope.4);
            rope.6.follow(&rope.5);
            rope.7.follow(&rope.6);
            rope.8.follow(&rope.7);
            rope.9.follow(&rope.8);
            tail_visited.insert(rope.9.clone());
        }
    }

    tail_visited.len() as i32
}
