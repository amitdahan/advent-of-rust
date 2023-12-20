use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, newline},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};
use num::integer::lcm;
use std::collections::{HashMap, VecDeque};

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = include_str!("example_1.in");
    const EXAMPLE_2: &str = include_str!("example_2.in");

    mod part1 {
        use super::*;

        #[test]
        fn it_solves_example_1() {
            let result = solve_part1(EXAMPLE_1);
            assert_eq!(result, 32000000);
        }

        #[test]
        fn it_solves_example_2() {
            let result = solve_part1(EXAMPLE_2);
            assert_eq!(result, 11687500);
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Module<'a> {
    Broadcast,
    FlipFlop(bool),
    Conjunction(HashMap<&'a str, Pulse>),
    Untyped,
}

fn parse(input: &str) -> IResult<&str, (HashMap<&str, Module>, HashMap<&str, Vec<&str>>)> {
    let (input, rows) = separated_list1(
        newline,
        separated_pair(
            alt((
                map(tag("broadcaster"), |_| ("broadcaster", Module::Broadcast)),
                map(preceded(char('%'), alpha1), |name| {
                    (name, Module::FlipFlop(false))
                }),
                map(preceded(char('&'), alpha1), |name| {
                    (name, Module::Conjunction(HashMap::new()))
                }),
                map(alpha1, |name| (name, Module::Untyped)),
            )),
            tag(" -> "),
            separated_list1(tag(", "), alpha1),
        ),
    )(input)?;

    let mut modules = rows
        .iter()
        .map(|((name, module), _)| (*name, module.clone()))
        .collect::<HashMap<_, _>>();

    let configurations = rows
        .iter()
        .map(|((name, _), destinations)| (*name, destinations.clone()))
        .collect::<HashMap<_, _>>();

    for (name, destinations) in &configurations {
        for destination in destinations {
            if !modules.contains_key(destination) {
                modules.insert(destination, Module::Untyped);
            }

            if let Some(Module::Conjunction(map)) = modules.get_mut(destination) {
                map.insert(name, Pulse::Low);
            }
        }
    }

    Ok((input, (modules, configurations)))
}

fn push_button<'a>(
    modules: &mut HashMap<&str, Module<'a>>,
    configurations: &HashMap<&str, Vec<&'a str>>,
    rx_conjunction: Option<&'a str>,
) -> (u32, u32, Option<&'a str>) {
    let mut low = 0;
    let mut high = 0;
    let mut conj_input = None;

    let mut q = VecDeque::from([("button", "broadcaster", Pulse::Low)]);

    while let Some((prev_name, curr_name, pulse)) = q.pop_back() {
        match pulse {
            Pulse::Low => low += 1,
            Pulse::High => high += 1,
        }

        if let Some(rx_conjunction) = rx_conjunction {
            if curr_name == rx_conjunction && pulse == Pulse::High {
                conj_input = Some(prev_name);
            }
        }

        let module = modules.get_mut(curr_name).unwrap();
        let destinations = configurations.get(curr_name);

        match (module, pulse) {
            (Module::Broadcast, pulse) => destinations
                .unwrap()
                .iter()
                .for_each(|destination| q.push_front((curr_name, destination, pulse.clone()))),

            (Module::FlipFlop(_), Pulse::High) => (),
            (Module::FlipFlop(state), Pulse::Low) => {
                destinations.unwrap().iter().for_each(|destination| {
                    q.push_front((
                        curr_name,
                        destination,
                        if *state { Pulse::Low } else { Pulse::High },
                    ))
                });
                *state = !*state;
            }

            (Module::Conjunction(inputs), pulse) => {
                *inputs.get_mut(prev_name).unwrap() = pulse;
                let next_pulse = if inputs.values().all(|pulse| pulse == &Pulse::High) {
                    Pulse::Low
                } else {
                    Pulse::High
                };
                destinations.unwrap().iter().for_each(|destination| {
                    q.push_front((curr_name, destination, next_pulse.clone()))
                });
            }

            (Module::Untyped, _) => (),
        }
    }

    (low, high, conj_input)
}

pub fn solve_part1(input: &str) -> u32 {
    let (_, (mut modules, configurations)) = parse(input).unwrap();

    let mut total_low = 0;
    let mut total_high = 0;
    for _ in 0..1000 {
        let (low, high, _) = push_button(&mut modules, &configurations, None);
        total_low += low;
        total_high += high;
    }

    total_low * total_high
}

pub fn solve_part2(input: &str) -> u128 {
    let (_, (mut modules, configurations)) = parse(input).unwrap();

    let rx_conjunction = configurations.iter().find_map(|(name, destinations)| {
        if destinations == &["rx"] {
            Some(*name)
        } else {
            None
        }
    });

    let input_count = match modules.get(rx_conjunction.unwrap()).unwrap() {
        Module::Conjunction(map) => map.len(),
        _ => panic!(),
    };

    let mut cycles = HashMap::new();

    for i in 0.. {
        let (_, _, conj_input) = push_button(&mut modules, &configurations, rx_conjunction);

        if let Some(conj_input) = conj_input {
            if !cycles.contains_key(conj_input) {
                cycles.insert(conj_input, i + 1);
            }
        }

        if cycles.len() == input_count {
            break;
        }
    }

    cycles.into_values().reduce(lcm).unwrap()
}
