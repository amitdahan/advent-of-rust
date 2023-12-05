use std::ops::Range;

use nom::{
    bytes::complete::tag,
    character::complete::{newline, u64},
    error::Error,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
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
            assert_eq!(result, 35);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn it_solves_example() {
            let result = solve_part2(EXAMPLE);
            assert_eq!(result, 46);
        }
    }
}

type RangeMap = (Range<u64>, Range<u64>);

fn parse_range_map(input: &str) -> IResult<&str, RangeMap> {
    let (input, dest_start) = u64(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, src_start) = u64(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, len) = u64(input)?;

    Ok((
        input,
        (src_start..(src_start + len), dest_start..(dest_start + len)),
    ))
}

fn parse_range_maps(input: &str) -> IResult<&str, Vec<Vec<RangeMap>>> {
    let (input, seed_to_soil) = preceded(
        tag("\n\nseed-to-soil map:\n"),
        separated_list1(newline, parse_range_map),
    )(input)?;
    let (input, soil_to_fertilizer) = preceded(
        tag("\n\nsoil-to-fertilizer map:\n"),
        separated_list1(newline, parse_range_map),
    )(input)?;
    let (input, fertilizer_to_water) = preceded(
        tag("\n\nfertilizer-to-water map:\n"),
        separated_list1(newline, parse_range_map),
    )(input)?;
    let (input, water_to_light) = preceded(
        tag("\n\nwater-to-light map:\n"),
        separated_list1(newline, parse_range_map),
    )(input)?;
    let (input, light_to_temperature) = preceded(
        tag("\n\nlight-to-temperature map:\n"),
        separated_list1(newline, parse_range_map),
    )(input)?;
    let (input, temperature_to_humidity) = preceded(
        tag("\n\ntemperature-to-humidity map:\n"),
        separated_list1(newline, parse_range_map),
    )(input)?;
    let (input, humidity_to_location) = preceded(
        tag("\n\nhumidity-to-location map:\n"),
        separated_list1(newline, parse_range_map),
    )(input)?;

    Ok((
        input,
        vec![
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        ],
    ))
}

fn parse_range(input: &str) -> IResult<&str, Range<u64>> {
    let (input, (start, len)) = separated_pair(u64, tag(" "), u64)(input)?;
    Ok((input, start..(start + len)))
}

fn map_range(range: &Range<u64>, range_maps: &[RangeMap]) -> Vec<Range<u64>> {
    let mut ranges = vec![];
    let mut start = range.start;

    while start < range.end {
        if let Some((src_range, dest_range)) = range_maps
            .iter()
            .find(|(src_range, _)| src_range.contains(&start))
        {
            let end = std::cmp::min(range.end, src_range.end);
            ranges.push(
                dest_range.start + (start - src_range.start)
                    ..dest_range.start + (end - src_range.start),
            );
            start = end;
        } else if let Some((src_range, _)) = range_maps
            .iter()
            .find(|(src_range, _)| (start..range.end).contains(&src_range.start))
        {
            let end = src_range.start;
            ranges.push(start..end);
            start = end;
        } else {
            ranges.push(start..range.end);
            start = range.end;
        }
    }

    ranges
}

fn solve(ranges: &[Range<u64>], range_maps: &Vec<Vec<RangeMap>>) -> u64 {
    let mut ranges = ranges.to_vec();

    for range_map in range_maps {
        ranges = ranges
            .iter()
            .flat_map(|range| map_range(range, range_map))
            .collect();
    }

    ranges
        .into_iter()
        .map(|Range { start, .. }| start)
        .min()
        .unwrap()
}

pub fn solve_part1(input: &str) -> u64 {
    let (input, seeds) = preceded(
        tag::<&str, &str, Error<&str>>("seeds: "),
        separated_list1(tag(" "), u64),
    )(input)
    .unwrap();
    let (_, range_maps) = parse_range_maps(input).unwrap();

    solve(
        &seeds
            .iter()
            .map(|&value| value..(value + 1))
            .collect::<Vec<_>>(),
        &range_maps,
    )
}

pub fn solve_part2(input: &str) -> u64 {
    let (input, ranges) = preceded(
        tag::<&str, &str, Error<&str>>("seeds: "),
        separated_list1(tag(" "), parse_range),
    )(input)
    .unwrap();
    let (_, range_maps) = parse_range_maps(input).unwrap();

    solve(&ranges, &range_maps)
}
