use nom::{
    bytes::complete::tag,
    character::complete::{i32, line_ending},
    multi::separated_list1,
    sequence::preceded,
    IResult,
};
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
            let result = solve_part1(EXAMPLE, 10);
            assert_eq!(result, 26);
        }

        #[test]
        fn it_solves_input() {
            let result = solve_part1(INPUT, 2_000_000);
            assert_eq!(result, 5_508_234);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn it_solves_example() {
            let result = solve_part2(EXAMPLE, 20);
            assert_eq!(result, 56_000_011);
        }

        #[test]
        fn it_solves_input() {
            let result = solve_part2(INPUT, 4_000_000);
            assert_eq!(result, 10_457_634_860_779);
        }
    }
}

type Position = (i32, i32);
type Reading = (Position, Position);

fn position(input: &str) -> IResult<&str, Position> {
    let (input, _) = tag("x=")(input)?;
    let (input, x) = i32(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, y) = i32(input)?;

    Ok((input, (x, y)))
}
fn beacon(input: &str) -> IResult<&str, Position> {
    // "closest beacon is at x=-2, y=15"
    preceded(tag("closest beacon is at "), position)(input)
}
fn sensor(input: &str) -> IResult<&str, Position> {
    // "Sensor at x=2, y=18: "
    preceded(tag("Sensor at "), position)(input)
}
fn reading(input: &str) -> IResult<&str, Reading> {
    let (input, sensor) = sensor(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, beacon) = beacon(&input)?;

    Ok((input, (sensor, beacon)))
}
fn parse_readings(input: &str) -> Vec<Reading> {
    separated_list1(line_ending, reading)(input).unwrap().1
}

fn manhattan_distance(a: &Position, b: &Position) -> u32 {
    ((a.0 - b.0).abs() + (a.1 - b.1).abs()) as u32
}

pub fn solve_part1(input: &str, y: i32) -> u32 {
    let sensors = parse_readings(input);
    let beacons: HashSet<Position> = sensors.iter().map(|(_, beacon)| beacon).cloned().collect();

    sensors
        .iter()
        .filter_map(|(sensor, beacon)| {
            let distance = manhattan_distance(sensor, beacon);

            let min_y = sensor.1 - distance as i32;
            let max_y = sensor.1 + distance as i32;

            if min_y <= y && y <= max_y {
                Some((sensor, distance))
            } else {
                None
            }
        })
        .flat_map(|(sensor, distance)| {
            let y_distance = (y - sensor.1).abs() as u32;
            let x_distance = distance - y_distance;

            (sensor.0 - x_distance as i32..=sensor.0 + x_distance as i32).map(move |x| (x, y))
        })
        .filter(|pos| !beacons.contains(pos))
        .collect::<HashSet<Position>>()
        .len() as u32
}

pub fn solve_part2(input: &str, search_space: i32) -> i64 {
    let sensors = parse_readings(input);

    let sensor_distances: Vec<(Position, u32)> = sensors
        .iter()
        .map(|(sensor, beacon)| (sensor.clone(), manhattan_distance(sensor, beacon)))
        .collect();

    let search_bounds = ((0, 0), (search_space, search_space));

    let mut possibilities: HashSet<Position> = sensor_distances
        .iter()
        .map(|(pos, distance)| get_perimeter(pos, *distance as i32, &search_bounds))
        .flatten()
        .collect();

    for ((x, y), distance) in &sensor_distances {
        possibilities.retain(|pos| manhattan_distance(pos, &(*x, *y)) > *distance);
    }

    for (_, beacon) in &sensors {
        possibilities.remove(beacon);
    }

    dbg!(&possibilities);

    let (x, y) = possibilities.iter().next().unwrap();

    *x as i64 * 4_000_000 as i64 + *y as i64
}

fn get_perimeter(pos: &Position, distance: i32, bounds: &(Position, Position)) -> Vec<Position> {
    let (x, y) = pos;

    let mut perimeter = Vec::new();

    let distance = distance + 1;

    for i in 0..distance {
        let pos_on_top_left_diagonal = (x - distance + i, y - i);
        if in_bounds(&pos_on_top_left_diagonal, bounds) {
            perimeter.push(pos_on_top_left_diagonal);
        }

        let pos_on_top_right_diagonal = (x + i, y - distance + i);
        if in_bounds(&pos_on_top_right_diagonal, bounds) {
            perimeter.push(pos_on_top_right_diagonal);
        }

        let pos_on_bottom_right_diagonal = (x + distance - i, y + i);
        if in_bounds(&pos_on_bottom_right_diagonal, bounds) {
            perimeter.push(pos_on_bottom_right_diagonal);
        }

        let pos_on_bottom_left_diagonal = (x - i, y + distance - i);
        if in_bounds(&pos_on_bottom_left_diagonal, bounds) {
            perimeter.push(pos_on_bottom_left_diagonal);
        }
    }

    perimeter
}

fn in_bounds(pos: &Position, bounds: &(Position, Position)) -> bool {
    pos.0 >= bounds.0 .0 && pos.0 <= bounds.1 .0 && pos.1 >= bounds.0 .1 && pos.1 <= bounds.1 .1
}
