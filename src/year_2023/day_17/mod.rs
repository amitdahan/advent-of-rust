use std::collections::{BinaryHeap, HashMap, HashSet};

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = include_str!("example_1.in");
    const EXAMPLE_2: &str = include_str!("example_2.in");

    mod part1 {
        use super::*;

        #[test]
        fn it_solves_example() {
            let result = solve_part1(EXAMPLE_1);
            assert_eq!(result, 102);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn it_solves_example_1() {
            let result = solve_part2(EXAMPLE_1);
            assert_eq!(result, 94);
        }

        #[test]
        fn it_solves_example_2() {
            let result = solve_part2(EXAMPLE_2);
            assert_eq!(result, 71);
        }
    }
}

fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| l.chars().map(|d| d.to_digit(10).unwrap() as u8).collect())
        .collect()
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash, Ord, PartialOrd)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash, Ord, PartialOrd)]
struct State {
    x: usize,
    y: usize,
    dir: Direction,
    count: u8,
}

fn next_states1(map: &[Vec<u8>], state: &State) -> Vec<State> {
    let directions = match state.dir {
        Direction::Up => [Direction::Left, Direction::Up, Direction::Right],
        Direction::Down => [Direction::Right, Direction::Down, Direction::Left],
        Direction::Left => [Direction::Down, Direction::Left, Direction::Up],
        Direction::Right => [Direction::Up, Direction::Right, Direction::Down],
    };

    directions
        .into_iter()
        .filter_map(|dir| {
            let (dx, dy) = match dir {
                Direction::Up => (0, -1),
                Direction::Down => (0, 1),
                Direction::Left => (-1, 0),
                Direction::Right => (1, 0),
            };

            let (next_x, next_y) = (state.x as i32 + dx, state.y as i32 + dy);
            if next_x < 0
                || next_y < 0
                || next_x >= map[0].len() as i32
                || next_y >= map.len() as i32
            {
                return None;
            }

            let (next_x, next_y) = (next_x as usize, next_y as usize);

            if dir == state.dir && state.count >= 3 {
                return None;
            }

            Some(State {
                x: next_x,
                y: next_y,
                dir,
                count: if dir == state.dir { state.count + 1 } else { 1 },
            })
        })
        .collect()
}
fn next_states2(map: &[Vec<u8>], state: &State) -> Vec<State> {
    let directions = match state.dir {
        Direction::Up => [Direction::Left, Direction::Up, Direction::Right],
        Direction::Down => [Direction::Right, Direction::Down, Direction::Left],
        Direction::Left => [Direction::Down, Direction::Left, Direction::Up],
        Direction::Right => [Direction::Up, Direction::Right, Direction::Down],
    };

    directions
        .into_iter()
        .filter_map(|dir| {
            let (dx, dy) = match dir {
                Direction::Up => (0, -1),
                Direction::Down => (0, 1),
                Direction::Left => (-1, 0),
                Direction::Right => (1, 0),
            };

            let (next_x, next_y) = (state.x as i32 + dx, state.y as i32 + dy);
            if next_x < 0
                || next_y < 0
                || next_x >= map[0].len() as i32
                || next_y >= map.len() as i32
            {
                return None;
            }

            let (next_x, next_y) = (next_x as usize, next_y as usize);

            // Min 4 moves in the same direction before turning
            if dir != state.dir && state.count < 4 {
                return None;
            }

            // Max 10 moves in the same direction
            if dir == state.dir && state.count >= 10 {
                return None;
            }

            Some(State {
                x: next_x,
                y: next_y,
                dir,
                count: if dir == state.dir { state.count + 1 } else { 1 },
            })
        })
        .collect()
}

fn shortest_paths(
    map: &[Vec<u8>],
    next_states: fn(&[Vec<u8>], &State) -> Vec<State>,
) -> HashMap<State, u32> {
    let initial_state_r = State {
        x: 0,
        y: 0,
        dir: Direction::Right,
        count: 0,
    };
    let initial_state_d = State {
        x: 0,
        y: 0,
        dir: Direction::Right,
        count: 0,
    };

    let mut visited = HashSet::new();
    let mut distances = HashMap::from([(initial_state_r, 0), (initial_state_d, 0)]);
    let mut q = BinaryHeap::from([(0i32, initial_state_r), (0i32, initial_state_d)]);

    while let Some((_, state)) = q.pop() {
        if !visited.insert(state) {
            continue;
        }

        let heat_loss = *distances.get(&state).unwrap();
        for next_state in next_states(map, &state) {
            let next_heat_loss = heat_loss + map[next_state.y][next_state.x] as u32;
            if next_heat_loss < *distances.get(&next_state).unwrap_or(&u32::MAX) {
                distances.insert(next_state, next_heat_loss);
                q.push((
                    // Negate so we use BinaryHeap as a min-heap
                    -(next_heat_loss as i32),
                    next_state,
                ));
            }
        }
    }

    distances
}

pub fn solve_part1(input: &str) -> u32 {
    let map = parse(input);

    *shortest_paths(&map, next_states1)
        .iter()
        .filter(|(&State { x, y, .. }, _)| x == map[0].len() - 1 && y == map.len() - 1)
        .map(|(_, d)| d)
        .min()
        .unwrap()
}

pub fn solve_part2(input: &str) -> u32 {
    let map = parse(input);

    *shortest_paths(&map, next_states2)
        .iter()
        .filter(|(&State { x, y, count, .. }, _)| {
            x == map[0].len() - 1 && y == map.len() - 1 && count >= 4
        })
        .map(|(_, d)| d)
        .min()
        .unwrap()
}
