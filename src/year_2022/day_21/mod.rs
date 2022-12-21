use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, i64, newline},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, terminated},
    IResult,
};
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.in");
    const INPUT: &str = include_str!("input.in");

    #[test]
    fn it_parses() {
        let data = parse(EXAMPLE);
        assert_eq!(
            data,
            HashMap::from_iter([
                ("root", Entry::Add("pppw", "sjmn")),
                ("dbpl", Entry::Num(5)),
                ("cczh", Entry::Add("sllz", "lgvd")),
                ("zczc", Entry::Num(2)),
                ("ptdq", Entry::Sub("humn", "dvpt")),
                ("dvpt", Entry::Num(3)),
                ("lfqf", Entry::Num(4)),
                ("humn", Entry::Num(5)),
                ("ljgn", Entry::Num(2)),
                ("sjmn", Entry::Mul("drzm", "dbpl")),
                ("sllz", Entry::Num(4)),
                ("pppw", Entry::Div("cczh", "lfqf")),
                ("lgvd", Entry::Mul("ljgn", "ptdq")),
                ("drzm", Entry::Sub("hmdt", "zczc")),
                ("hmdt", Entry::Num(32)),
            ])
        );
    }

    mod part1 {
        use super::*;

        #[test]
        fn it_solves_example() {
            let result = solve_part1(EXAMPLE);
            assert_eq!(result, 152);
        }

        #[test]
        fn it_solves_input() {
            let result = solve_part1(INPUT);
            assert_eq!(result, 84_244_467_642_604);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn it_solves_example() {
            let result = solve_part2(EXAMPLE);
            assert_eq!(result, 301);
        }

        #[test]
        fn it_solves_input() {
            let result = solve_part2(INPUT);
            assert_eq!(result, 3_759_569_926_192);
        }
    }
}

fn entry(input: &str) -> IResult<&str, Entry> {
    let (input, left) = alpha1(input)?;
    let (input, op) = delimited(
        tag(" "),
        alt((tag("+"), tag("-"), tag("*"), tag("/"))),
        tag(" "),
    )(input)?;
    let (input, right) = alpha1(input)?;

    Ok((
        input,
        match op {
            "+" => Entry::Add(left, right),
            "-" => Entry::Sub(left, right),
            "*" => Entry::Mul(left, right),
            "/" => Entry::Div(left, right),
            _ => unreachable!(),
        },
    ))
}
fn line(input: &str) -> IResult<&str, (&str, Entry)> {
    let (input, name) = terminated(alpha1, tag(": "))(input)?;
    let (input, entry) = alt((map(i64, |n| Entry::Num(n)), entry))(input)?;

    Ok((input, (name, entry)))
}
fn parse(input: &str) -> HashMap<&str, Entry> {
    let (_, entries) = separated_list1(newline, line)(input).unwrap();

    HashMap::from_iter(entries)
}

#[derive(Debug, PartialEq)]
enum Entry<'a> {
    Add(&'a str, &'a str),
    Sub(&'a str, &'a str),
    Mul(&'a str, &'a str),
    Div(&'a str, &'a str),
    Num(i64),
}

fn resolve_monkey(
    name: &str,
    data: &HashMap<&str, Entry>,
    cache: &mut HashMap<String, i64>,
) -> i64 {
    if let Some(&n) = cache.get(name) {
        return n;
    }

    let result = match data.get(name).unwrap() {
        Entry::Add(ref left, ref right) => {
            resolve_monkey(left, data, cache) + resolve_monkey(right, data, cache)
        }
        Entry::Sub(ref left, ref right) => {
            resolve_monkey(left, data, cache) - resolve_monkey(right, data, cache)
        }
        Entry::Mul(ref left, ref right) => {
            resolve_monkey(left, data, cache) * resolve_monkey(right, data, cache)
        }
        Entry::Div(ref left, ref right) => {
            resolve_monkey(left, data, cache) / resolve_monkey(right, data, cache)
        }
        Entry::Num(n) => *n,
    };

    cache.insert(name.to_string(), result.clone());

    result
}

pub fn solve_part1(input: &str) -> i64 {
    let data = parse(input);
    let mut cache = HashMap::new();
    resolve_monkey("root", &data, &mut cache)
}

pub fn solve_part2(input: &str) -> i64 {
    let data = parse(input);

    let (left, right) = match data.get("root") {
        Some(&Entry::Add(ref left, ref right)) => (left, right),
        _ => unreachable!(),
    };

    let mut needs_humn_cache = HashMap::new();
    let left_needs_humn = needs_humn(&data, left, &mut needs_humn_cache);

    let mut resolve_cache = HashMap::new();

    let value = if left_needs_humn {
        resolve_monkey(right, &data, &mut HashMap::new())
    } else {
        resolve_monkey(left, &data, &mut HashMap::new())
    };

    resolve_humn(
        if left_needs_humn { left } else { right },
        value,
        &data,
        &mut resolve_cache,
        &mut needs_humn_cache,
    )
    .unwrap()
}

fn needs_humn(data: &HashMap<&str, Entry>, name: &str, cache: &HashMap<String, bool>) -> bool {
    if let Some(&needs) = cache.get(name) {
        return needs;
    }

    match data.get(name).unwrap() {
        Entry::Add("humn", _) => true,
        Entry::Add(_, "humn") => true,
        Entry::Sub("humn", _) => true,
        Entry::Sub(_, "humn") => true,
        Entry::Mul("humn", _) => true,
        Entry::Mul(_, "humn") => true,
        Entry::Div("humn", _) => true,
        Entry::Div(_, "humn") => true,

        Entry::Add(left, right) => {
            needs_humn(data, &left, cache) || needs_humn(data, &right, cache)
        }
        Entry::Sub(left, right) => {
            needs_humn(data, &left, cache) || needs_humn(data, &right, cache)
        }
        Entry::Mul(left, right) => {
            needs_humn(data, &left, cache) || needs_humn(data, &right, cache)
        }
        Entry::Div(left, right) => {
            needs_humn(data, &left, cache) || needs_humn(data, &right, cache)
        }
        Entry::Num(_) => false,
    }
}

fn resolve_humn(
    name: &str,
    value: i64,
    data: &HashMap<&str, Entry>,
    resolve_cache: &mut HashMap<String, i64>,
    needs_humn_cache: &mut HashMap<String, bool>,
) -> Option<i64> {
    match data.get(name).unwrap() {
        Entry::Add("humn", monkey) => Some(value - resolve_monkey(monkey, data, resolve_cache)),
        Entry::Add(monkey, "humn") => Some(value - resolve_monkey(monkey, data, resolve_cache)),

        Entry::Sub("humn", monkey) => Some(value + resolve_monkey(monkey, data, resolve_cache)),
        Entry::Sub(monkey, "humn") => Some(resolve_monkey(monkey, data, resolve_cache) - value),

        Entry::Mul("humn", monkey) => Some(value / resolve_monkey(monkey, data, resolve_cache)),
        Entry::Mul(monkey, "humn") => Some(value / resolve_monkey(monkey, data, resolve_cache)),

        Entry::Div("humn", monkey) => Some(value * resolve_monkey(monkey, data, resolve_cache)),
        Entry::Div(monkey, "humn") => Some(resolve_monkey(monkey, data, resolve_cache) / value),

        Entry::Add(left, right) if needs_humn(data, left, needs_humn_cache) => resolve_humn(
            left,
            value - resolve_monkey(right, data, resolve_cache),
            data,
            resolve_cache,
            needs_humn_cache,
        ),
        Entry::Add(left, right) if needs_humn(data, right, needs_humn_cache) => resolve_humn(
            right,
            value - resolve_monkey(left, data, resolve_cache),
            data,
            resolve_cache,
            needs_humn_cache,
        ),

        Entry::Sub(left, right) if needs_humn(data, left, needs_humn_cache) => resolve_humn(
            left,
            value + resolve_monkey(right, data, resolve_cache),
            data,
            resolve_cache,
            needs_humn_cache,
        ),
        Entry::Sub(left, right) if needs_humn(data, right, needs_humn_cache) => resolve_humn(
            right,
            resolve_monkey(left, data, resolve_cache) - value,
            data,
            resolve_cache,
            needs_humn_cache,
        ),

        Entry::Mul(left, right) if needs_humn(data, left, needs_humn_cache) => resolve_humn(
            left,
            value / resolve_monkey(right, data, resolve_cache),
            data,
            resolve_cache,
            needs_humn_cache,
        ),
        Entry::Mul(left, right) if needs_humn(data, right, needs_humn_cache) => resolve_humn(
            right,
            value / resolve_monkey(left, data, resolve_cache),
            data,
            resolve_cache,
            needs_humn_cache,
        ),

        Entry::Div(left, right) if needs_humn(data, left, needs_humn_cache) => resolve_humn(
            left,
            value * resolve_monkey(right, data, resolve_cache),
            data,
            resolve_cache,
            needs_humn_cache,
        ),
        Entry::Div(left, right) if needs_humn(data, right, needs_humn_cache) => resolve_humn(
            right,
            resolve_monkey(left, data, resolve_cache) / value,
            data,
            resolve_cache,
            needs_humn_cache,
        ),

        _ => None,
    }
}
