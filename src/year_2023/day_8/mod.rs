use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::newline,
    combinator::map,
    multi::{count, many1, separated_list1},
    sequence::{delimited, separated_pair},
    IResult,
};
use num::integer::lcm;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = include_str!("example_1.in");
    const EXAMPLE_2: &str = include_str!("example_2.in");
    const EXAMPLE_3: &str = include_str!("example_3.in");

    mod part1 {
        use super::*;

        #[test]
        fn it_solves_example_1() {
            let result = solve_part1(EXAMPLE_1);
            assert_eq!(result, 2);
        }

        #[test]
        fn it_solves_example_2() {
            let result = solve_part1(EXAMPLE_2);
            assert_eq!(result, 6);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn it_solves_example_3() {
            let result = solve_part2(EXAMPLE_3);
            assert_eq!(result, 6);
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Left,
    Right,
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(alt((
        map(tag("L"), |_| Instruction::Left),
        map(tag("R"), |_| Instruction::Right),
    )))(input)
}
fn parse_vertex(input: &str) -> IResult<&str, &str> {
    take(3usize)(input)
}
fn parse_edge(input: &str) -> IResult<&str, (&str, (&str, &str))> {
    separated_pair(
        parse_vertex,
        tag(" = "),
        delimited(
            tag("("),
            separated_pair(parse_vertex, tag(", "), parse_vertex),
            tag(")"),
        ),
    )(input)
}
fn parse_graph(input: &str) -> IResult<&str, HashMap<&str, (&str, &str)>> {
    let (input, edges) = separated_list1(newline, parse_edge)(input)?;

    Ok((input, HashMap::from_iter(edges.into_iter())))
}
fn parse_input(input: &str) -> IResult<&str, (Vec<Instruction>, HashMap<&str, (&str, &str)>)> {
    separated_pair(parse_instructions, count(newline, 2), parse_graph)(input)
}

fn solve(
    graph: &HashMap<&str, (&str, &str)>,
    instructions: &[Instruction],
    start: &str,
    predicate: impl Fn(&str) -> bool,
) -> u64 {
    instructions
        .iter()
        .cycle()
        .scan(start, |curr, instruction| {
            let (left, right) = graph.get(*curr).unwrap();
            *curr = match instruction {
                Instruction::Left => left,
                Instruction::Right => right,
            };
            Some(*curr)
        })
        .take_while(|curr| !predicate(curr))
        .count() as u64
        + 1
}

pub fn solve_part1(input: &str) -> u64 {
    let (_, (instructions, graph)) = parse_input(input).unwrap();
    solve(&graph, &instructions, "AAA", |k| k == "ZZZ")
}

pub fn solve_part2(input: &str) -> u64 {
    let (_, (instructions, graph)) = parse_input(input).unwrap();

    graph
        .keys()
        .filter(|k| k.chars().nth(2).unwrap() == 'A')
        .map(|k| solve(&graph, &instructions, k, |k| k.ends_with("Z")))
        .reduce(lcm)
        .unwrap()
}
