use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{newline, u32},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::cmp::Ordering;

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.in");

    mod part1 {
        use super::*;

        #[test]
        fn it_solves_example() {
            let result = solve_part1(EXAMPLE);
            assert_eq!(result, 6440);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn it_solves_example() {
            let result = solve_part2(EXAMPLE);
            assert_eq!(result, 5905);
        }
    }
}

type Card = u8; // 2..=14

#[derive(Debug, Eq, PartialEq)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (x, y) if x == y => Ordering::Equal,
            (Self::FiveOfAKind, _) => Ordering::Greater,
            (_, Self::FiveOfAKind) => Ordering::Less,
            (Self::FourOfAKind, _) => Ordering::Greater,
            (_, Self::FourOfAKind) => Ordering::Less,
            (Self::FullHouse, _) => Ordering::Greater,
            (_, Self::FullHouse) => Ordering::Less,
            (Self::ThreeOfAKind, _) => Ordering::Greater,
            (_, Self::ThreeOfAKind) => Ordering::Less,
            (Self::TwoPair, _) => Ordering::Greater,
            (_, Self::TwoPair) => Ordering::Less,
            (Self::OnePair, _) => Ordering::Greater,
            (_, Self::OnePair) => Ordering::Less,
            _ => unreachable!(),
        }
    }
}
impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    cards: [Card; 5],
}

impl Hand {
    fn hand_type(&self) -> HandType {
        let mut counts = [0; 15]; // 0 and 1 are unused
        for card in &self.cards {
            counts[*card as usize] += 1;
        }
        let counts = &counts.iter().sorted().rev().collect::<Vec<_>>()[0..5];

        match counts {
            [5, _, _, _, _] => HandType::FiveOfAKind,
            [4, 1, _, _, _] => HandType::FourOfAKind,
            [3, 2, _, _, _] => HandType::FullHouse,
            [3, 1, 1, _, _] => HandType::ThreeOfAKind,
            [2, 2, 1, _, _] => HandType::TwoPair,
            [2, 1, 1, 1, _] => HandType::OnePair,
            [1, 1, 1, 1, 1] => HandType::HighCard,
            _ => unreachable!(),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_type = self.hand_type();
        let other_type = other.hand_type();
        if self_type == other_type {
            self.cards.cmp(&other.cards)
        } else {
            self_type.cmp(&other_type)
        }
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Eq, PartialEq)]
struct HandWithJokers {
    cards: [Card; 5],
}

impl From<&Hand> for HandWithJokers {
    fn from(hand: &Hand) -> Self {
        Self {
            cards: [
                if hand.cards[0] == 11 {
                    1
                } else {
                    hand.cards[0]
                },
                if hand.cards[1] == 11 {
                    1
                } else {
                    hand.cards[1]
                },
                if hand.cards[2] == 11 {
                    1
                } else {
                    hand.cards[2]
                },
                if hand.cards[3] == 11 {
                    1
                } else {
                    hand.cards[3]
                },
                if hand.cards[4] == 11 {
                    1
                } else {
                    hand.cards[4]
                },
            ],
        }
    }
}

impl HandWithJokers {
    fn hand_type(&self) -> HandType {
        let mut counts = [0; 15]; // 0 and 11 are unused
        for card in &self.cards {
            counts[*card as usize] += 1;
        }
        let jokers = counts[1];
        let counts = &counts.iter().sorted().rev().collect::<Vec<_>>()[0..5];

        match (counts, jokers) {
            ([5, _, _, _, _], _) => HandType::FiveOfAKind,
            ([4, 1, _, _, _], 4) => HandType::FiveOfAKind,
            ([4, 1, _, _, _], 1) => HandType::FiveOfAKind,
            ([4, 1, _, _, _], 0) => HandType::FourOfAKind,
            ([3, 2, _, _, _], 3) => HandType::FiveOfAKind,
            ([3, 2, _, _, _], 2) => HandType::FiveOfAKind,
            ([3, 2, _, _, _], 0) => HandType::FullHouse,
            ([3, 1, 1, _, _], 3) => HandType::FourOfAKind,
            ([3, 1, 1, _, _], 1) => HandType::FourOfAKind,
            ([3, 1, 1, _, _], 0) => HandType::ThreeOfAKind,
            ([2, 2, 1, _, _], 2) => HandType::FourOfAKind,
            ([2, 2, 1, _, _], 1) => HandType::FullHouse,
            ([2, 2, 1, _, _], 0) => HandType::TwoPair,
            ([2, 1, 1, 1, _], 2) => HandType::ThreeOfAKind,
            ([2, 1, 1, 1, _], 1) => HandType::ThreeOfAKind,
            ([2, 1, 1, 1, _], 0) => HandType::OnePair,
            ([1, 1, 1, 1, 1], 1) => HandType::OnePair,
            ([1, 1, 1, 1, 1], 0) => HandType::HighCard,
            _ => unreachable!(),
        }
    }
}

impl Ord for HandWithJokers {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_type = self.hand_type();
        let other_type = other.hand_type();
        if self_type == other_type {
            self.cards.cmp(&other.cards)
        } else {
            self_type.cmp(&other_type)
        }
    }
}
impl PartialOrd for HandWithJokers {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_numeric_card(input: &str) -> IResult<&str, Card> {
    let (input, card) = alt((
        map(tag("2"), |_| 2),
        map(tag("3"), |_| 3),
        map(tag("4"), |_| 4),
        map(tag("5"), |_| 5),
        map(tag("6"), |_| 6),
        map(tag("7"), |_| 7),
        map(tag("8"), |_| 8),
        map(tag("9"), |_| 9),
    ))(input)?;

    Ok((input, card))
}
fn parse_char_card(input: &str) -> IResult<&str, Card> {
    let (input, card) = alt((
        map(tag("T"), |_| 10),
        map(tag("J"), |_| 11),
        map(tag("Q"), |_| 12),
        map(tag("K"), |_| 13),
        map(tag("A"), |_| 14),
    ))(input)?;

    Ok((input, card))
}
fn parse_card(input: &str) -> IResult<&str, Card> {
    alt((parse_numeric_card, parse_char_card))(input)
}
fn parse_hand(input: &str) -> IResult<&str, Hand> {
    let (input, card1) = parse_card(input)?;
    let (input, card2) = parse_card(input)?;
    let (input, card3) = parse_card(input)?;
    let (input, card4) = parse_card(input)?;
    let (input, card5) = parse_card(input)?;
    Ok((
        input,
        Hand {
            cards: [card1, card2, card3, card4, card5],
        },
    ))
}
fn parse_line(input: &str) -> IResult<&str, (Hand, u32)> {
    separated_pair(parse_hand, tag(" "), u32)(input)
}
fn parse_input(input: &str) -> IResult<&str, Vec<(Hand, u32)>> {
    separated_list1(newline, parse_line)(input)
}

pub fn solve_part1(input: &str) -> u32 {
    parse_input(input)
        .unwrap()
        .1
        .iter()
        .sorted()
        .enumerate()
        .map(|(i, (_, score))| score * (i as u32 + 1))
        .sum()
}

pub fn solve_part2(input: &str) -> u32 {
    parse_input(input)
        .unwrap()
        .1
        .iter()
        .map(|(hand, score)| (HandWithJokers::from(hand), *score))
        .sorted()
        .enumerate()
        .map(|(i, (_, score))| score * (i as u32 + 1))
        .sum()
}
