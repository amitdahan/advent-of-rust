use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{newline, u32},
    combinator::map,
    multi::{many1, separated_list1},
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
            assert_eq!(result, 21);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn it_solves_example() {
            let result = solve_part2(EXAMPLE);
            assert_eq!(result, 525152);
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Status {
    Operational,
    Damaged,
    Unknown,
}

type Record = (Vec<Status>, Vec<u32>);

fn parse_status(input: &str) -> IResult<&str, Status> {
    alt((
        map(tag("."), |_| Status::Operational),
        map(tag("#"), |_| Status::Damaged),
        map(tag("?"), |_| Status::Unknown),
    ))(input)
}
fn parse(input: &str) -> IResult<&str, Vec<Record>> {
    separated_list1(
        newline,
        separated_pair(
            many1(parse_status),
            tag(" "),
            separated_list1(tag(","), u32),
        ),
    )(input)
}

type CacheKey = (usize, usize, Option<u32>);

fn count_arrangements(
    statuses: &[Status],
    groups: &[u32],
    consecutive: Option<u32>,
    cache: &mut HashMap<CacheKey, u64>,
) -> u64 {
    if let Some(curr) = statuses.first() {
        use Status::*;

        let key = (statuses.len(), groups.len(), consecutive);

        if let Some(value) = cache.get(&key) {
            return *value;
        }

        let value = match (curr, &consecutive, groups.first()) {
            (_, Some(_), None) => 0,
            (Operational, Some(consecutive), Some(group)) if consecutive != group => 0,
            (Operational, None, _) => count_arrangements(&statuses[1..], groups, None, cache),
            (Damaged, _, None) => 0,
            (Damaged, Some(consecutive), Some(group)) if consecutive >= group => 0,
            (Damaged, Some(consecutive), Some(_)) => {
                count_arrangements(&statuses[1..], groups, Some(consecutive + 1), cache)
            }
            (Damaged, None, _) => count_arrangements(&statuses[1..], groups, Some(1), cache),
            (Unknown, None, None) => count_arrangements(&statuses[1..], groups, None, cache),
            (Unknown, None, Some(_)) => {
                count_arrangements(&statuses[1..], groups, None, cache)
                    + count_arrangements(&statuses[1..], groups, Some(1), cache)
            }
            (Unknown, Some(consecutive), Some(group)) if consecutive > group => 0,
            (Unknown, Some(consecutive), Some(group)) if consecutive < group => {
                count_arrangements(&statuses[1..], groups, Some(consecutive + 1), cache)
            }
            (_, Some(_), Some(_)) => count_arrangements(&statuses[1..], &groups[1..], None, cache),
        };
        cache.insert(key, value);
        value
    } else {
        match (consecutive, groups.len(), groups.first()) {
            (None, 0, _) => 1,
            (None, _, None) => 1,
            (Some(consecutive), 1, Some(group)) if consecutive == *group => 1,
            _ => 0,
        }
    }
}

fn unfold((statuses, groups): &Record) -> Record {
    let statuses = [
        statuses.to_vec(),
        vec![Status::Unknown],
        statuses.to_vec(),
        vec![Status::Unknown],
        statuses.to_vec(),
        vec![Status::Unknown],
        statuses.to_vec(),
        vec![Status::Unknown],
        statuses.to_vec(),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>();

    let groups = [
        groups.to_vec(),
        groups.to_vec(),
        groups.to_vec(),
        groups.to_vec(),
        groups.to_vec(),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>();

    (statuses, groups)
}
fn solve(rows: &[Record]) -> u64 {
    rows.iter()
        .map(|(statuses, groups)| count_arrangements(statuses, groups, None, &mut HashMap::new()))
        .sum()
}
pub fn solve_part1(input: &str) -> u64 {
    let (_, rows) = parse(input).unwrap();
    solve(&rows)
}
pub fn solve_part2(input: &str) -> u64 {
    let (_, rows) = parse(input).unwrap();
    let rows = rows.iter().map(unfold).collect::<Vec<_>>();

    solve(&rows)
}
