use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{newline, u8},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    IResult,
};

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
            assert_eq!(result, 8);
        }

        #[test]
        fn it_solves_input() {
            let result = solve_part1(INPUT);
            assert_eq!(result, 0);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn it_solves_example() {
            let result = solve_part2(EXAMPLE);
            assert_eq!(result, 2286);
        }

        #[test]
        fn it_solves_input() {
            let result = solve_part2(INPUT);
            assert_eq!(result, 0);
        }
    }
}

#[derive(Debug)]
struct Round {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Debug)]
struct Game {
    id: u8,
    rounds: Vec<Round>,
}

#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

fn parse_color(input: &str) -> IResult<&str, Color> {
    alt((
        map(tag("red"), |_| Color::Red),
        map(tag("green"), |_| Color::Green),
        map(tag("blue"), |_| Color::Blue),
    ))(input)
}

fn parse_round(input: &str) -> IResult<&str, Round> {
    let (input, amounts) =
        separated_list1(tag(", "), separated_pair(u8, tag(" "), parse_color))(input)?;

    Ok((
        input,
        Round {
            red: amounts
                .iter()
                .find(|(_, color)| color == &Color::Red)
                .map(|(amount, _)| *amount)
                .unwrap_or(0),
            green: amounts
                .iter()
                .find(|(_, color)| color == &Color::Green)
                .map(|(amount, _)| *amount)
                .unwrap_or(0),
            blue: amounts
                .iter()
                .find(|(_, color)| color == &Color::Blue)
                .map(|(amount, _)| *amount)
                .unwrap_or(0),
        },
    ))
}

fn parse_rounds(input: &str) -> IResult<&str, Vec<Round>> {
    separated_list1(tag("; "), parse_round)(input)
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, id) = delimited(tag("Game "), u8, tag(": "))(input)?;
    let (input, rounds) = parse_rounds(input)?;

    Ok((input, Game { id, rounds }))
}

fn parse_games(input: &str) -> IResult<&str, Vec<Game>> {
    separated_list1(newline, parse_game)(input)
}

const RED_THRESHOLD: u8 = 12;
const GREEN_THRESHOLD: u8 = 13;
const BLUE_THRESHOLD: u8 = 14;

pub fn solve_part1(input: &str) -> u32 {
    let mut sum = 0;

    let (_, games) = parse_games(input).unwrap();

    for game in games {
        if game.rounds.iter().all(|round| {
            round.red <= RED_THRESHOLD
                && round.green <= GREEN_THRESHOLD
                && round.blue <= BLUE_THRESHOLD
        }) {
            sum += game.id as u32;
        }
    }

    sum
}

pub fn solve_part2(input: &str) -> u32 {
    let mut sum = 0;

    let (_, games) = parse_games(input).unwrap();

    for game in games {
        let red = game.rounds.iter().map(|round| round.red).max().unwrap_or(0);
        let green = game
            .rounds
            .iter()
            .map(|round| round.green)
            .max()
            .unwrap_or(0);
        let blue = game
            .rounds
            .iter()
            .map(|round| round.blue)
            .max()
            .unwrap_or(0);

        sum += red as u32 * green as u32 * blue as u32;
    }

    sum
}
