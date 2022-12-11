use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace1, u64},
    multi::separated_list1,
    sequence::{delimited, preceded},
    IResult, Parser,
};
use std::collections::VecDeque;

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.in");
    const INPUT: &str = include_str!("input.in");

    #[test]
    fn it_parses() {
        let (_, monkeys) = monkeys(EXAMPLE).unwrap();
        assert_eq!(
            monkeys,
            vec![
                Monkey {
                    items: vec![79, 98].into(),
                    operation: MonkeyOperation::Multiply(19),
                    test_divisible_by: 23,
                    monkey_if_true: 2,
                    monkey_if_false: 3
                },
                Monkey {
                    items: vec![54, 65, 75, 74].into(),
                    operation: MonkeyOperation::Add(6),
                    test_divisible_by: 19,
                    monkey_if_true: 2,
                    monkey_if_false: 0
                },
                Monkey {
                    items: vec![79, 60, 97].into(),
                    operation: MonkeyOperation::Square,
                    test_divisible_by: 13,
                    monkey_if_true: 1,
                    monkey_if_false: 3
                },
                Monkey {
                    items: vec![74].into(),
                    operation: MonkeyOperation::Add(3),
                    test_divisible_by: 17,
                    monkey_if_true: 0,
                    monkey_if_false: 1
                },
            ]
        );
    }

    mod part1 {
        use super::*;

        #[test]
        fn it_solves_example() {
            let result = solve_part1(EXAMPLE);
            assert_eq!(result, 10605);
        }

        #[test]
        fn it_solves_input() {
            let result = solve_part1(INPUT);
            assert_eq!(result, 117624);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn it_solves_example() {
            let result = solve_part2(EXAMPLE);

            assert_eq!(result, 2713310158);
        }

        #[test]
        fn it_solves_input() {
            let result = solve_part2(INPUT);
            assert_eq!(result, 16792940265);
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum MonkeyOperation {
    Add(u64),
    Multiply(u64),
    Square,
}

#[derive(Debug, Eq, PartialEq)]
struct Monkey {
    items: VecDeque<u64>,
    operation: MonkeyOperation,
    test_divisible_by: u64,
    monkey_if_true: usize,
    monkey_if_false: usize,
}

fn monkey_operation(input: &str) -> IResult<&str, MonkeyOperation> {
    enum MonkeyOperationValue {
        Old,
        Value(u64),
    }

    let (input, _) = tag("old")(input)?;
    let (input, op) = delimited(tag(" "), alt((tag("*"), tag("+"))), tag(" "))(input)?;
    let (input, value) = alt((
        tag("old").map(|_| MonkeyOperationValue::Old),
        u64.map(MonkeyOperationValue::Value),
    ))(input)?;

    match (op, value) {
        ("*", MonkeyOperationValue::Old) => Ok((input, MonkeyOperation::Square)),
        ("*", MonkeyOperationValue::Value(value)) => Ok((input, MonkeyOperation::Multiply(value))),
        ("+", MonkeyOperationValue::Value(value)) => Ok((input, MonkeyOperation::Add(value))),
        _ => unreachable!(),
    }
}

fn monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, _) = delimited(tag("Monkey "), u64, tag(":"))(input)?;
    let (input, _) = multispace1(input)?;

    let (input, items) = delimited(
        tag("Starting items: "),
        separated_list1(tag(", "), u64),
        multispace1,
    )(input)?;
    let (input, operation) =
        delimited(tag("Operation: new = "), monkey_operation, multispace1)(input)?;
    let (input, test_divisible_by) =
        delimited(tag("Test: divisible by "), u64, multispace1)(input)?;
    let (input, monkey_if_true) =
        delimited(tag("If true: throw to monkey "), u64, multispace1)(input)?;
    let (input, monkey_if_false) = preceded(tag("If false: throw to monkey "), u64)(input)?;

    Ok((
        input,
        Monkey {
            items: items.into(),
            operation,
            test_divisible_by,
            monkey_if_true: monkey_if_true as usize,
            monkey_if_false: monkey_if_false as usize,
        },
    ))
}

fn monkeys(input: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list1(tag("\n\n"), monkey)(input)
}

pub fn solve_part1(input: &str) -> u32 {
    let (_, mut monkeys) = monkeys(input).unwrap();
    let mut monkey_inspections = vec![0; monkeys.len()];
    const ROUNDS: usize = 20;

    for _ in 0..ROUNDS {
        for i in 0..monkeys.len() {
            let monkey = &monkeys[i];
            let passes = {
                monkey
                    .items
                    .iter()
                    .map(|worry_level| {
                        let new_worry_level = match monkey.operation {
                            MonkeyOperation::Add(value) => worry_level + value,
                            MonkeyOperation::Multiply(value) => worry_level * value,
                            MonkeyOperation::Square => worry_level * worry_level,
                        } / 3;

                        let target_monkey = if new_worry_level % monkey.test_divisible_by == 0 {
                            monkey.monkey_if_true
                        } else {
                            monkey.monkey_if_false
                        };

                        (target_monkey, new_worry_level)
                    })
                    .collect::<Vec<(usize, u64)>>()
            };

            monkey_inspections[i] += monkey.items.len();
            monkeys[i].items.clear();
            for (target_monkey, new_worry_level) in passes {
                monkeys[target_monkey].items.push_back(new_worry_level);
            }
        }
    }

    monkey_inspections.sort_by(|a, b| b.cmp(a));
    monkey_inspections[0] as u32 * monkey_inspections[1] as u32
}

pub fn solve_part2(input: &str) -> u64 {
    let (_, mut monkeys) = monkeys(input).unwrap();
    let mut monkey_inspections = vec![0; monkeys.len()];
    const ROUNDS: usize = 10_000;

    let product = monkeys
        .iter()
        .map(|monkey| monkey.test_divisible_by)
        .product::<u64>();

    for _ in 0..ROUNDS {
        for i in 0..monkeys.len() {
            let monkey = &monkeys[i];
            let passes = {
                monkey
                    .items
                    .iter()
                    .map(|worry_level| {
                        let worry_level = match monkey.operation {
                            MonkeyOperation::Add(value) => worry_level + value,
                            MonkeyOperation::Multiply(value) => worry_level * value,
                            MonkeyOperation::Square => worry_level * worry_level,
                        } % product;

                        let target_monkey = if worry_level % monkey.test_divisible_by == 0 {
                            monkey.monkey_if_true
                        } else {
                            monkey.monkey_if_false
                        };

                        (target_monkey, worry_level)
                    })
                    .collect::<Vec<(usize, u64)>>()
            };

            monkey_inspections[i] += monkey.items.len();
            monkeys[i].items.clear();
            for (target_monkey, new_worry_level) in passes {
                monkeys[target_monkey].items.push_back(new_worry_level);
            }
        }
    }

    monkey_inspections.sort_by(|a, b| b.cmp(a));
    monkey_inspections[0] as u64 * monkey_inspections[1] as u64
}
