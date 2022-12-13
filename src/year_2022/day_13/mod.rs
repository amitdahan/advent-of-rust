use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{newline, u32},
    combinator::map,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    IResult,
};
use std::cmp::Ordering;

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.in");
    const INPUT: &str = include_str!("input.in");

    #[test]
    fn it_parses() {
        let pairs = parse_pairs(EXAMPLE);
        assert_eq!(
            pairs,
            vec![
                (
                    Item::ArrayItem(vec![
                        Item::SingleItem(1),
                        Item::SingleItem(1),
                        Item::SingleItem(3),
                        Item::SingleItem(1),
                        Item::SingleItem(1),
                    ]),
                    Item::ArrayItem(vec![
                        Item::SingleItem(1),
                        Item::SingleItem(1),
                        Item::SingleItem(5),
                        Item::SingleItem(1),
                        Item::SingleItem(1),
                    ])
                ),
                (
                    Item::ArrayItem(vec![
                        Item::ArrayItem(vec![Item::SingleItem(1),]),
                        Item::ArrayItem(vec![
                            Item::SingleItem(2),
                            Item::SingleItem(3),
                            Item::SingleItem(4),
                        ]),
                    ]),
                    Item::ArrayItem(vec![
                        Item::ArrayItem(vec![Item::SingleItem(1),]),
                        Item::SingleItem(4),
                    ])
                ),
                (
                    Item::ArrayItem(vec![Item::SingleItem(9),]),
                    Item::ArrayItem(vec![Item::ArrayItem(vec![
                        Item::SingleItem(8),
                        Item::SingleItem(7),
                        Item::SingleItem(6),
                    ]),])
                ),
                (
                    Item::ArrayItem(vec![
                        Item::ArrayItem(vec![Item::SingleItem(4), Item::SingleItem(4),]),
                        Item::SingleItem(4),
                        Item::SingleItem(4),
                    ]),
                    Item::ArrayItem(vec![
                        Item::ArrayItem(vec![Item::SingleItem(4), Item::SingleItem(4),]),
                        Item::SingleItem(4),
                        Item::SingleItem(4),
                        Item::SingleItem(4),
                    ]),
                ),
                (
                    Item::ArrayItem(vec![
                        Item::SingleItem(7),
                        Item::SingleItem(7),
                        Item::SingleItem(7),
                        Item::SingleItem(7),
                    ]),
                    Item::ArrayItem(vec![
                        Item::SingleItem(7),
                        Item::SingleItem(7),
                        Item::SingleItem(7),
                    ]),
                ),
                (
                    Item::ArrayItem(vec![]),
                    Item::ArrayItem(vec![Item::SingleItem(3),])
                ),
                (
                    Item::ArrayItem(vec![Item::ArrayItem(vec![Item::ArrayItem(vec![])]),]),
                    Item::ArrayItem(vec![Item::ArrayItem(vec![]),])
                ),
                (
                    Item::ArrayItem(vec![
                        Item::SingleItem(1),
                        Item::ArrayItem(vec![
                            Item::SingleItem(2),
                            Item::ArrayItem(vec![
                                Item::SingleItem(3),
                                Item::ArrayItem(vec![
                                    Item::SingleItem(4),
                                    Item::ArrayItem(vec![
                                        Item::SingleItem(5),
                                        Item::SingleItem(6),
                                        Item::SingleItem(7),
                                    ]),
                                ]),
                            ]),
                        ]),
                        Item::SingleItem(8),
                        Item::SingleItem(9),
                    ]),
                    Item::ArrayItem(vec![
                        Item::SingleItem(1),
                        Item::ArrayItem(vec![
                            Item::SingleItem(2),
                            Item::ArrayItem(vec![
                                Item::SingleItem(3),
                                Item::ArrayItem(vec![
                                    Item::SingleItem(4),
                                    Item::ArrayItem(vec![
                                        Item::SingleItem(5),
                                        Item::SingleItem(6),
                                        Item::SingleItem(0),
                                    ]),
                                ]),
                            ]),
                        ]),
                        Item::SingleItem(8),
                        Item::SingleItem(9),
                    ]),
                )
            ]
        );
    }

    #[test]
    fn it_compares() {
        let pairs = parse_pairs(EXAMPLE);
        let comparisons = pairs.iter().map(|(a, b)| a <= b).collect::<Vec<_>>();
        assert_eq!(
            comparisons,
            vec![true, true, false, true, false, true, false, false]
        );
    }

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
            assert_eq!(result, 5808);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn it_solves_example() {
            let result = solve_part2(EXAMPLE);
            assert_eq!(result, 140);
        }

        #[test]
        fn it_solves_input() {
            let result = solve_part2(INPUT);
            assert_eq!(result, 22713);
        }
    }
}

type ArrayItem = Vec<Item>;

#[derive(Debug, PartialEq, Eq, Ord)]
enum Item {
    ArrayItem(ArrayItem),
    SingleItem(u32),
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Item::SingleItem(a), Item::SingleItem(b)) => a.partial_cmp(b),
            (Item::ArrayItem(a), Item::ArrayItem(b)) => a.partial_cmp(b),
            (Item::SingleItem(a), Item::ArrayItem(_)) => {
                Item::ArrayItem(vec![Item::SingleItem(*a)]).partial_cmp(other)
            }
            (Item::ArrayItem(_), Item::SingleItem(b)) => {
                self.partial_cmp(&Item::ArrayItem(vec![Item::SingleItem(*b)]))
            }
        }
    }
}

type ItemPair = (Item, Item);

fn array_item(input: &str) -> IResult<&str, Item> {
    map(
        delimited(tag("["), separated_list0(tag(","), item), tag("]")),
        Item::ArrayItem,
    )(input)
}

fn single_item(input: &str) -> IResult<&str, Item> {
    map(u32, Item::SingleItem)(input)
}

fn item(input: &str) -> IResult<&str, Item> {
    alt((array_item, single_item))(input)
}

fn item_pair(input: &str) -> IResult<&str, ItemPair> {
    separated_pair(item, newline, item)(input)
}

fn parse_pairs(input: &str) -> Vec<ItemPair> {
    let (_, pairs) = separated_list1(tag("\n\n"), item_pair)(input).unwrap();

    pairs
}

pub fn solve_part1(input: &str) -> u32 {
    let pairs = parse_pairs(input);

    pairs
        .iter()
        .enumerate()
        .filter_map(|(i, (a, b))| if a <= b { Some(i as u32 + 1) } else { None })
        .sum()
}

pub fn solve_part2(input: &str) -> u32 {
    let pairs = parse_pairs(input);
    let divider_packets = parse_pairs("[[2]]\n[[6]]");

    let mut items = pairs
        .iter()
        .chain(divider_packets.iter())
        .flat_map(|(a, b)| vec![a, b])
        .collect::<Vec<_>>();

    items.sort();

    let (first, second) = &divider_packets[0];
    items
        .iter()
        .enumerate()
        .filter_map(|(i, item)| {
            if *item == first || *item == second {
                Some(i as u32 + 1)
            } else {
                None
            }
        })
        .product()
}
