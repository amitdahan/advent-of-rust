use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, newline, u32},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    IResult,
};
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.in");

    mod part1 {
        use super::*;

        #[test]
        fn it_solves_example() {
            let result = solve_part1(EXAMPLE);
            assert_eq!(result, 19114);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn it_solves_example() {
            let result = solve_part2(EXAMPLE);
            assert_eq!(result, 167409079868000);
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Attribute {
    X,
    M,
    A,
    S,
}

#[derive(Debug, Copy, Clone)]
enum Rule {
    Gt(Attribute, u32),
    Lt(Attribute, u32),
    Goto,
}

impl Rule {
    fn not(&self) -> Rule {
        match self {
            Rule::Gt(attr, value) => Rule::Lt(*attr, *value + 1),
            Rule::Lt(attr, value) => Rule::Gt(*attr, *value - 1),
            Rule::Goto => unreachable!("Goto rule cannot be negated"),
        }
    }
}

#[derive(Debug)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Part {
    fn rating(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }

    fn get_attr(&self, attr: &Attribute) -> u32 {
        match attr {
            Attribute::X => self.x,
            Attribute::M => self.m,
            Attribute::A => self.a,
            Attribute::S => self.s,
        }
    }
}

type Workflow<'a> = Vec<(Rule, &'a str)>;

fn parse_workflow_name(input: &str) -> IResult<&str, &str> {
    alpha1(input)
}
fn parse_attribute(input: &str) -> IResult<&str, Attribute> {
    alt((
        map(char('x'), |_| Attribute::X),
        map(char('m'), |_| Attribute::M),
        map(char('a'), |_| Attribute::A),
        map(char('s'), |_| Attribute::S),
    ))(input)
}
fn parse_rule(input: &str) -> IResult<&str, (Rule, &str)> {
    alt((
        map(
            separated_pair(
                parse_attribute,
                char('>'),
                separated_pair(u32, char(':'), parse_workflow_name),
            ),
            |(attribute, (value, goto))| (Rule::Gt(attribute, value), goto),
        ),
        map(
            separated_pair(
                parse_attribute,
                char('<'),
                separated_pair(u32, char(':'), parse_workflow_name),
            ),
            |(attribute, (value, goto))| (Rule::Lt(attribute, value), goto),
        ),
        map(parse_workflow_name, |goto| (Rule::Goto, goto)),
    ))(input)
}
fn parse_rules(input: &str) -> IResult<&str, Workflow> {
    separated_list1(char(','), parse_rule)(input)
}
fn parse_workflow(input: &str) -> IResult<&str, (&str, Workflow)> {
    let (input, name) = parse_workflow_name(input)?;
    let (input, rules) = delimited(char('{'), parse_rules, char('}'))(input)?;
    Ok((input, (name, rules)))
}
fn parse_workflows(input: &str) -> IResult<&str, HashMap<&str, Workflow>> {
    let (input, workflows) = separated_list1(newline, parse_workflow)(input)?;
    Ok((input, workflows.into_iter().collect()))
}
fn parse_part(input: &str) -> IResult<&str, Part> {
    let (input, _) = char('{')(input)?;
    let (input, x) = delimited(tag("x="), u32, char(','))(input)?;
    let (input, m) = delimited(tag("m="), u32, char(','))(input)?;
    let (input, a) = delimited(tag("a="), u32, char(','))(input)?;
    let (input, s) = delimited(tag("s="), u32, char('}'))(input)?;
    Ok((input, Part { x, m, a, s }))
}
fn parse_parts(input: &str) -> IResult<&str, Vec<Part>> {
    separated_list1(newline, parse_part)(input)
}
fn parse(input: &str) -> IResult<&str, (HashMap<&str, Workflow>, Vec<Part>)> {
    separated_pair(parse_workflows, tag("\n\n"), parse_parts)(input)
}

fn is_accepted(workflows: &HashMap<&str, Workflow>, part: &Part) -> bool {
    let mut curr = "in";

    loop {
        if curr == "A" {
            return true;
        }
        if curr == "R" {
            return false;
        }

        for (rule, goto) in workflows.get(curr).unwrap() {
            match rule {
                Rule::Gt(attr, value) if part.get_attr(attr) > *value => {
                    curr = goto;
                    break;
                }
                Rule::Lt(attr, value) if part.get_attr(attr) < *value => {
                    curr = goto;
                    break;
                }
                Rule::Goto => {
                    curr = goto;
                    break;
                }
                _ => {}
            }
        }
    }
}
pub fn solve_part1(input: &str) -> u32 {
    let (_, (workflows, parts)) = parse(input).unwrap();
    parts
        .iter()
        .filter(|part| is_accepted(&workflows, part))
        .map(|part| part.rating())
        .sum()
}

fn reject_paths(
    workflows: &HashMap<&str, Vec<(Rule, &str)>>,
    curr: &str,
    path: &mut Vec<Rule>,
    paths: &mut Vec<Vec<Rule>>,
) {
    if curr == "R" {
        paths.push(path.clone());
        return;
    }
    if curr == "A" {
        return;
    }

    let workflow = workflows.get(curr).unwrap();

    for (rule, goto) in workflow {
        match rule {
            Rule::Gt(_, _) => {
                path.push(*rule);
                reject_paths(workflows, goto, path, paths);
                path.pop();
                path.push(rule.not());
            }
            Rule::Lt(_, _) => {
                path.push(*rule);
                reject_paths(workflows, goto, path, paths);
                path.pop();
                path.push(rule.not());
            }
            Rule::Goto => {
                reject_paths(workflows, goto, path, paths);
            }
        }
    }
    for _ in workflow
        .iter()
        .filter(|(rule, _)| !matches!(rule, Rule::Goto))
    {
        path.pop();
    }
}

pub fn solve_part2(input: &str) -> u64 {
    let (_, (workflows, _)) = parse(input).unwrap();

    let mut paths = vec![];
    reject_paths(&workflows, "in", &mut vec![], &mut paths);

    let mut combinations = 4000 * 4000 * 4000 * 4000;

    for path in paths {
        let mut min_x = 1;
        let mut max_x = 4000;
        let mut min_m = 1;
        let mut max_m = 4000;
        let mut min_a = 1;
        let mut max_a = 4000;
        let mut min_s = 1;
        let mut max_s = 4000;

        for rule in path {
            match rule {
                Rule::Gt(Attribute::X, value) => {
                    min_x = min_x.max(value + 1);
                }
                Rule::Lt(Attribute::X, value) => {
                    max_x = max_x.min(value - 1);
                }
                Rule::Gt(Attribute::M, value) => {
                    min_m = min_m.max(value + 1);
                }
                Rule::Lt(Attribute::M, value) => {
                    max_m = max_m.min(value - 1);
                }
                Rule::Gt(Attribute::A, value) => {
                    min_a = min_a.max(value + 1);
                }
                Rule::Lt(Attribute::A, value) => {
                    max_a = max_a.min(value - 1);
                }
                Rule::Gt(Attribute::S, value) => {
                    min_s = min_s.max(value + 1);
                }
                Rule::Lt(Attribute::S, value) => {
                    max_s = max_s.min(value - 1);
                }
                _ => {}
            }
        }

        combinations -= (max_x - min_x + 1) as u64
            * (max_m - min_m + 1) as u64
            * (max_a - min_a + 1) as u64
            * (max_s - min_s + 1) as u64;
    }

    combinations
}
