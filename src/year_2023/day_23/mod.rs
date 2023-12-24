use std::collections::{HashMap, HashSet, VecDeque};

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
            assert_eq!(result, 94);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn it_solves_example() {
            let result = solve_part2(EXAMPLE);
            assert_eq!(result, 154);
        }
    }
}

type Coords = (usize, usize);

#[derive(Debug, PartialEq, Eq, Hash)]
enum Cell {
    Wall,
    Empty,
    RightSlope,
    DownSlope,
}

fn parse(input: &str, can_climb: bool) -> Vec<Vec<Cell>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match (c, can_climb) {
                    ('#', _) => Cell::Wall,
                    ('.', _) => Cell::Empty,
                    ('>', true) => Cell::Empty,
                    ('>', false) => Cell::RightSlope,
                    ('v', true) => Cell::Empty,
                    ('v', false) => Cell::DownSlope,
                    _ => panic!("Invalid cell"),
                })
                .collect_vec()
        })
        .collect_vec()
}

fn compress_graph(edges: &mut HashMap<Coords, Vec<(Coords, usize)>>, start: &Coords) {
    let mut q = VecDeque::from([(*start, *start)]);
    let mut visited = HashSet::new();

    while let Some((prev, curr)) = q.pop_front() {
        if !visited.insert(curr) {
            continue;
        }

        if edges.get(&curr).unwrap().len() == 2 {
            let (destination, neighbor_cost) = edges
                .get(&curr)
                .unwrap()
                .iter()
                .find(|(neighbor, _)| *neighbor != prev)
                .unwrap()
                .clone();

            let (_, curr_cost) = edges
                .get(&prev)
                .unwrap()
                .iter()
                .find(|(neighbor, _)| *neighbor == curr)
                .unwrap()
                .clone();

            edges
                .get_mut(&prev)
                .unwrap()
                .retain(|(neighbor, _)| *neighbor != curr);
            edges
                .get_mut(&prev)
                .unwrap()
                .push((destination, curr_cost + neighbor_cost));

            edges
                .get_mut(&destination)
                .unwrap()
                .retain(|(neighbor, _)| *neighbor != curr);
            edges
                .get_mut(&destination)
                .unwrap()
                .push((prev, curr_cost + neighbor_cost));

            edges.remove(&curr);

            q.push_back((prev, destination));
        } else {
            for (neighbor, _) in &edges[&curr] {
                q.push_back((curr, *neighbor));
            }
        }
    }
}
fn build_graph(grid: &[Vec<Cell>], start: &Coords) -> HashMap<Coords, Vec<(Coords, usize)>> {
    let mut graph = HashMap::new();
    let w = grid[0].len();
    let h = grid.len();

    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == Cell::Wall {
                continue;
            }

            let mut neighbors = Vec::new();

            if matches!(cell, &Cell::Empty | &Cell::RightSlope)
                && x + 1 < w
                && grid[y][x + 1] != Cell::Wall
            {
                neighbors.push(((x + 1, y), 1usize));
            }
            if matches!(cell, &Cell::Empty | &Cell::DownSlope)
                && y + 1 < h
                && grid[y + 1][x] != Cell::Wall
            {
                neighbors.push(((x, y + 1), 1usize));
            }
            if cell == &Cell::Empty && x > 0 && grid[y][x - 1] != Cell::Wall {
                neighbors.push(((x - 1, y), 1usize));
            }
            if cell == &Cell::Empty && y > 0 && grid[y - 1][x] != Cell::Wall {
                neighbors.push(((x, y - 1), 1usize));
            }

            graph.insert((x, y), neighbors);
        }
    }

    compress_graph(&mut graph, start);

    graph
}

fn longest_path(
    edges: &HashMap<Coords, Vec<(Coords, usize)>>,
    start: &Coords,
    end: &Coords,
) -> usize {
    let mut q = VecDeque::from([(start, 0, HashSet::from([start]))]);
    let mut result = 0;

    while let Some((curr, cost, nodes)) = q.pop_front() {
        if curr == end {
            result = result.max(cost);
            continue;
        }

        for (neighbor, new_cost) in &edges[&curr] {
            if !nodes.contains(neighbor) {
                let mut new_nodes = nodes.clone();
                new_nodes.insert(neighbor);
                q.push_back((neighbor, cost + new_cost, new_nodes));
            }
        }
    }

    result
}

pub fn solve_part1(input: &str) -> usize {
    let grid = parse(input, false);
    let start = (1, 0);
    let end = (grid[0].len() - 2, grid.len() - 1);
    let edges = build_graph(&grid, &start);
    longest_path(&edges, &start, &end)
}

pub fn solve_part2(input: &str) -> usize {
    let grid = parse(input, true);
    let start = (1, 0);
    let end = (grid[0].len() - 2, grid.len() - 1);
    let edges = build_graph(&grid, &start);
    longest_path(&edges, &start, &end)
}
