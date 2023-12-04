use std::collections::HashSet;

use nom::{
    bytes::complete::tag,
    character::complete::{multispace1, newline, u8},
    multi::separated_list1,
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
            assert_eq!(result, 13);
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
            assert_eq!(result, 30);
        }

        #[test]
        fn it_solves_input() {
            let result = solve_part2(INPUT);
            assert_eq!(result, 0);
        }
    }
}

#[derive(Debug)]
struct Card {
    winning_numbers: Vec<u8>,
    rolled_numbers: Vec<u8>,
}

impl Card {
    fn matching_cards(&self) -> u8 {
        let winners = self.winning_numbers.iter().collect::<HashSet<_>>();

        self.rolled_numbers
            .iter()
            .filter(|n| winners.contains(n))
            .count() as u8
    }
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    let (input, _) = tag("Card")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = u8(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = multispace1(input)?;

    let (input, winning_numbers) = separated_list1(multispace1, u8)(input)?;
    let (input, _) = tag(" |")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, rolled_numbers) = separated_list1(multispace1, u8)(input)?;

    Ok((
        input,
        Card {
            winning_numbers,
            rolled_numbers,
        },
    ))
}

fn parse_cards(input: &str) -> IResult<&str, Vec<Card>> {
    separated_list1(newline, parse_card)(input)
}

pub fn solve_part1(input: &str) -> u32 {
    let (_, cards) = parse_cards(input).unwrap();

    cards
        .iter()
        .map(|c| c.matching_cards())
        .filter(|&c| c > 0)
        .map(|c| 2u32.pow((c - 1) as u32))
        .sum()
}

pub fn solve_part2(input: &str) -> u32 {
    let (_, cards) = parse_cards(input).unwrap();

    let mut scratchcards = vec![0; cards.len()];

    for (i, matches) in cards.iter().map(|c| c.matching_cards()).enumerate().rev() {
        scratchcards[i] = 1 + scratchcards[i + 1..i + 1 + matches as usize]
            .iter()
            .sum::<u32>();
    }

    scratchcards.iter().sum()
}
