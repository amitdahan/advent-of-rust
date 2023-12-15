use std::{array, collections::HashMap};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::u8,
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
            assert_eq!(result, 1320);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn it_solves_example() {
            let result = solve_part2(EXAMPLE);
            assert_eq!(result, 145);
        }
    }
}

fn parse_strings(input: &str) -> Vec<&str> {
    input.trim().split(',').collect()
}
fn hash(s: &str) -> u32 {
    let mut h = 0;

    for c in s.chars() {
        h += c as u32;
        h *= 17;
        h %= 256;
    }

    h
}
pub fn solve_part1(input: &str) -> u32 {
    parse_strings(input).iter().map(|s| hash(s)).sum()
}

#[derive(Debug)]
enum LensOp {
    Remove,
    Insert(u8),
}

type Step<'a> = (&'a str, LensOp);

fn parse_remove_step(input: &str) -> IResult<&str, Step> {
    let (input, label) = take_until("-")(input)?;
    let (input, _) = tag("-")(input)?;
    Ok((input, (label, LensOp::Remove)))
}
fn parse_insert_step(input: &str) -> IResult<&str, Step> {
    let (input, label) = take_until("=")(input)?;
    let (input, _) = tag("=")(input)?;
    let (input, focal_length) = u8(input)?;
    Ok((input, (label, LensOp::Insert(focal_length))))
}
fn parse_step(input: &str) -> IResult<&str, Step> {
    alt((parse_remove_step, parse_insert_step))(input)
}

pub fn solve_part2(input: &str) -> u32 {
    let steps: Vec<(&str, LensOp)> = parse_strings(input)
        .iter()
        .map(|s| parse_step(s).unwrap().1)
        .collect::<Vec<_>>();

    let mut map = HashMap::new();
    let mut boxes: [Vec<(&str, u8)>; 256] = array::from_fn(|_| vec![]);

    for (label, op) in &steps {
        let h = hash(label);
        let new_box = &mut boxes[h as usize];
        let curr_box_i = map.get(label);
        match (curr_box_i, op) {
            (Some(&curr_box_i), LensOp::Remove) if curr_box_i == h => {
                let i = new_box.iter().position(|(l, _)| l == label).unwrap();
                new_box.remove(i);
                map.remove(label);
            }
            (_, LensOp::Remove) => (),

            (Some(&curr_box_i), LensOp::Insert(focal_length)) if curr_box_i == h => {
                let i = new_box.iter().position(|(l, _)| l == label).unwrap();
                new_box[i] = (label, *focal_length);
                map.insert(*label, h);
            }

            (_, LensOp::Insert(focal_length)) => {
                new_box.push((label, *focal_length));
                map.insert(*label, h);
            }
        }
    }

    boxes
        .iter()
        .enumerate()
        .flat_map(|(box_i, b)| {
            b.iter()
                .enumerate()
                .map(|(lens_i, &(_, focal_length))| {
                    (box_i as u32 + 1) * (lens_i as u32 + 1) * focal_length as u32
                })
                .collect::<Vec<_>>()
        })
        .sum::<u32>()
}
