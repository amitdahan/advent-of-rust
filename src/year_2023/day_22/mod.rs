use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;
use nom::{
    character::complete::{char, newline, u32},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.in");

    mod part1 {
        use super::*;

        #[test]
        fn it_solves_example() {
            let result = solve_part1(EXAMPLE);
            assert_eq!(result, 5);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn it_solves_example() {
            let result = solve_part2(EXAMPLE);
            assert_eq!(result, 7);
        }
    }
}

type Coords = (usize, usize, usize);

fn parse_coords(input: &str) -> IResult<&str, Coords> {
    map(
        separated_pair(u32, char(','), separated_pair(u32, char(','), u32)),
        |(x, (y, z))| (x as usize, y as usize, z as usize),
    )(input)
}
fn parse_line(input: &str) -> IResult<&str, (Coords, Coords)> {
    separated_pair(parse_coords, char('~'), parse_coords)(input)
}
fn parse(input: &str) -> IResult<&str, Vec<(Coords, Coords)>> {
    separated_list1(newline, parse_line)(input)
}

fn intersects_x_y(
    ((x1, y1, _), (x2, y2, _)): &((usize, usize, usize), (usize, usize, usize)),
    ((xx1, yy1, _), (xx2, yy2, _)): &((usize, usize, usize), (usize, usize, usize)),
) -> bool {
    let min_x = x1.min(x2);
    let max_x = x1.max(x2);
    let min_y = y1.min(y2);
    let max_y = y1.max(y2);
    let x_range = min_x..=max_x;
    let y_range = min_y..=max_y;

    let min_xx = xx1.min(xx2);
    let max_xx = xx1.max(xx2);
    let min_yy = yy1.min(yy2);
    let max_yy = yy1.max(yy2);
    let xx_range = min_xx..=max_xx;
    let yy_range = min_yy..=max_yy;

    let x_intersects = x_range.contains(&min_xx)
        || x_range.contains(&max_xx)
        || xx_range.contains(&min_x)
        || xx_range.contains(&max_x);
    let y_intersects = y_range.contains(&min_yy)
        || y_range.contains(&max_yy)
        || yy_range.contains(&min_y)
        || yy_range.contains(&max_y);

    x_intersects && y_intersects
}

fn apply_gravity(bricks: &mut [(Coords, Coords)]) -> HashMap<usize, HashSet<usize>> {
    let mut supported_by: HashMap<usize, HashSet<usize>> = HashMap::new();

    bricks.sort_by(|((_, _, az1), (_, _, bz1)), ((_, _, az2), (_, _, bz2))| {
        az1.min(bz1).cmp(az2.min(bz2))
    });

    for i in 0..bricks.len() {
        let ((_, _, z1), (_, _, z2)) = bricks[i];
        let min_z = z1.min(z2);

        let supporting_bricks = (0..i)
            .rev()
            .filter(|j| intersects_x_y(&bricks[i], &bricks[*j]))
            .max_set_by_key(|j| {
                let ((_, _, zz1), (_, _, zz2)) = bricks[*j];
                zz1.max(zz2)
            });

        if let Some(supporting_brick_i) = supporting_bricks.first() {
            let ((_, _, zz1), (_, _, zz2)) = bricks[*supporting_brick_i];
            let z_delta = min_z - zz1.max(zz2) - 1;

            bricks[i].0 .2 -= z_delta;
            bricks[i].1 .2 -= z_delta;

            for supporting_brick_i in supporting_bricks {
                supported_by
                    .entry(i)
                    .or_default()
                    .insert(supporting_brick_i);
            }
        } else {
            bricks[i].0 .2 -= min_z - 1;
            bricks[i].1 .2 -= min_z - 1;
        }
    }

    supported_by
}

pub fn solve_part1(input: &str) -> u32 {
    let (_, mut bricks) = parse(input).unwrap();
    let supported_by = apply_gravity(&mut bricks);

    bricks
        .iter()
        .enumerate()
        .filter(|(i, _)| {
            supported_by
                .values()
                .all(|supports| !supports.contains(i) || supports.len() > 1)
        })
        .count() as u32
}

pub fn solve_part2(input: &str) -> u32 {
    let (_, mut bricks) = parse(input).unwrap();
    let supported_by = apply_gravity(&mut bricks);

    bricks
        .iter()
        .enumerate()
        .map(|(i, _)| {
            let mut fell: HashSet<usize> = HashSet::from([i]);
            let mut q = VecDeque::from([i]);

            while let Some(j) = q.pop_front() {
                fell.insert(j);
                for (supported, supports) in &supported_by {
                    if supports.difference(&fell).count() == 0 {
                        if fell.insert(*supported) {
                            q.push_back(*supported);
                        }
                    }
                }
            }

            fell.len() as u32 - 1
        })
        .sum()
}
