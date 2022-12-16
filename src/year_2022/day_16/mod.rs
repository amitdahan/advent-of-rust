use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, newline, u32},
    multi::separated_list1,
    sequence::{delimited, preceded},
    IResult,
};
use petgraph::{algo::floyd_warshall, prelude::DiGraphMap};
use std::collections::{HashMap, HashSet};

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.in");
    const INPUT: &str = include_str!("input.in");

    mod part1 {
        use super::*;

        #[test]
        fn it_solves_example() {
            let result = solve_part1(EXAMPLE);
            assert_eq!(result, 1651);
        }

        #[test]
        fn it_solves_input() {
            let result = solve_part1(INPUT);
            assert_eq!(result, 1862);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn it_solves_example() {
            let result = solve_part2(EXAMPLE);
            assert_eq!(result, 1707);
        }

        #[test]
        fn it_solves_input() {
            let result = solve_part2(INPUT);
            assert_eq!(result, 2422);
        }
    }
}

fn report_entry(input: &str) -> IResult<&str, (&str, u32, Vec<&str>)> {
    let (input, name) = delimited(tag("Valve "), alpha1, tag(" has "))(input)?;
    let (input, flow_rate) = delimited(tag("flow rate="), u32, tag("; "))(input)?;
    let (input, tunnels) = preceded(
        alt((
            tag("tunnels lead to valves "),
            tag("tunnel leads to valve "),
        )),
        separated_list1(tag(", "), alpha1),
    )(input)?;

    Ok((input, (name, flow_rate, tunnels)))
}
fn reports(input: &str) -> IResult<&str, Vec<(&str, u32, Vec<&str>)>> {
    separated_list1(newline, report_entry)(input)
}
fn parse_graph(input: &str) -> (DiGraphMap<&str, u32>, HashMap<&str, u32>) {
    let (_, entries) = reports(input).unwrap();
    let mut graph = DiGraphMap::new();
    let mut flow_rates = HashMap::new();

    for (valve, flow_rate, _) in &entries {
        graph.add_node(*valve);
        flow_rates.insert(*valve, *flow_rate);
    }

    for (valve, _, tunnels) in &entries {
        for tunnel in tunnels {
            graph.add_edge(*valve, *tunnel, 1);
        }
    }

    (graph, flow_rates)
}

fn get_max_total_flow(
    graph: &DiGraphMap<&str, u32>,
    flow_rates: &HashMap<&str, u32>,
    closed_valves: HashSet<&str>,
    distances: &HashMap<(&str, &str), u32>,
    curr: &str,
    time_left: u32,
) -> u32 {
    closed_valves
        .iter()
        .map(|valve| (valve, 1 + distances[&(curr, *valve)]))
        .filter(|(_, distance)| *distance < time_left)
        .map(|(target_valve, distance)| {
            let mut new_closed_valves = closed_valves.clone();
            new_closed_valves.remove(target_valve);

            let flow_rate = flow_rates[target_valve];
            let total_flow = (time_left - distance) * flow_rate;

            get_max_total_flow(
                graph,
                flow_rates,
                new_closed_valves,
                distances,
                target_valve,
                time_left - distance,
            ) + total_flow
        })
        .max()
        .unwrap_or(0)
}

pub fn solve_part1(input: &str) -> u32 {
    let (g, flow_rates) = parse_graph(input);
    let distances = floyd_warshall(&g, |_| 1).unwrap();

    get_max_total_flow(
        &g,
        &flow_rates,
        HashSet::from_iter(
            flow_rates
                .iter()
                .filter(|(_, flow_rate)| **flow_rate > 0)
                .map(|(valve, _)| *valve),
        ),
        &distances,
        "AA",
        30,
    )
}

pub fn solve_part2(input: &str) -> u32 {
    let (g, flow_rates) = parse_graph(input);
    let distances = floyd_warshall(&g, |_| 1).unwrap();

    let closed_valves: HashSet<&str> = HashSet::from_iter(
        flow_rates
            .iter()
            .filter(|(_, flow_rate)| **flow_rate > 0)
            .map(|(valve, _)| *valve),
    );

    let closed_valves_clone = closed_valves.clone();

    closed_valves
        .into_iter()
        .powerset()
        .map(|vec| vec.iter().copied().collect::<HashSet<_>>())
        .map(|human_set| {
            let elephant_valves = closed_valves_clone
                .difference(&human_set)
                .copied()
                .collect::<HashSet<_>>();

            (human_set, elephant_valves)
        })
        .map(|(human_valves, elephant_valves)| {
            get_max_total_flow(&g, &flow_rates, human_valves, &distances, "AA", 26)
                + get_max_total_flow(&g, &flow_rates, elephant_valves, &distances, "AA", 26)
        })
        .max()
        .unwrap_or(0)
}
