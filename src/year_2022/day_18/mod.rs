use nom::{
    bytes::complete::tag,
    character::complete::{i32, newline},
    multi::separated_list1,
    IResult,
};
use std::collections::{HashSet, VecDeque};

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.in");
    const INPUT: &str = include_str!("input.in");

    mod part1 {
        use super::*;

        #[test]
        fn it_solves_smaller_example() {
            let result = solve_part1("1,1,1\n2,1,1");
            assert_eq!(result, 10);
        }

        #[test]
        fn it_solves_example() {
            let result = solve_part1(EXAMPLE);
            assert_eq!(result, 64);
        }

        #[test]
        fn it_solves_input() {
            let result = solve_part1(INPUT);
            assert_eq!(result, 4370);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn it_solves_example() {
            let result = solve_part2(EXAMPLE);
            assert_eq!(result, 58);
        }

        #[test]
        fn it_solves_input() {
            let result = solve_part2(INPUT);
            assert_eq!(result, 2458);
        }
    }
}

fn parse_line(input: &str) -> IResult<&str, Point> {
    let (input, x) = i32(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, y) = i32(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, z) = i32(input)?;

    Ok((input, (x, y, z)))
}
fn parse(input: &str) -> Vec<Point> {
    let (_, coords) = separated_list1(newline, parse_line)(input).unwrap();

    coords
}

type Point = (i32, i32, i32);

fn cube_neighbors(&(x, y, z): &Point) -> [Point; 6] {
    [
        (x, y, z + 1),
        (x, y, z - 1),
        (x, y + 1, z),
        (x, y - 1, z),
        (x + 1, y, z),
        (x - 1, y, z),
    ]
}

pub fn solve_part1(input: &str) -> u32 {
    let cubes = parse(input).into_iter().collect::<HashSet<Point>>();

    let mut surface_area = 0;

    for cube in &cubes {
        for neighbor in cube_neighbors(cube) {
            if !cubes.contains(&neighbor) {
                surface_area += 1;
            }
        }
    }

    surface_area
}

fn is_outside(
    &(x, y, z): &Point,
    &((min_x, min_y, min_z), (max_x, max_y, max_z)): &(Point, Point),
) -> bool {
    x < min_x || x > max_x || y < min_y || y > max_y || z < min_z || z > max_z
}

fn leads_outside(cubes: &HashSet<Point>, from: &Point, bounds: &(Point, Point)) -> bool {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_front(*from);

    while let Some(point) = queue.pop_front() {
        if is_outside(&point, bounds) {
            return true;
        }

        for neighbor in cube_neighbors(&point) {
            if !visited.contains(&neighbor) && !cubes.contains(&neighbor) {
                visited.insert(neighbor);
                queue.push_back(neighbor);
            }
        }
    }

    false
}

pub fn solve_part2(input: &str) -> u32 {
    let cubes = parse(input).into_iter().collect::<HashSet<Point>>();

    let bounds = (
        (
            cubes.iter().map(|(x, _, _)| x).min().unwrap().to_owned(),
            cubes.iter().map(|(_, y, _)| y).min().unwrap().to_owned(),
            cubes.iter().map(|(_, _, z)| z).min().unwrap().to_owned(),
        ),
        (
            cubes.iter().map(|(x, _, _)| x).max().unwrap().to_owned(),
            cubes.iter().map(|(_, y, _)| y).max().unwrap().to_owned(),
            cubes.iter().map(|(_, _, z)| z).max().unwrap().to_owned(),
        ),
    );

    let mut surface_area = 0;

    for cube in &cubes {
        for neighbor in &cube_neighbors(cube) {
            if !cubes.contains(&neighbor) && leads_outside(&cubes, neighbor, &bounds) {
                surface_area += 1;
            }
        }
    }

    surface_area
}
