use std::{collections::HashMap, ops::Range};

use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, space0, space1, u32 as nom_u32},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
    IResult,
};

pub use error::{Error, Result};

mod error;

pub fn part1(input: impl AsRef<str>) -> u32 {
    let input = input.as_ref();
    let (_, cards) = cards(input).expect("cards to parse");
    cards.iter().map(Card::value).sum()
}

pub fn part2(input: impl AsRef<str>) -> u32 {
    let input = input.as_ref();
    let (_, cards) = cards(input).expect("cards to parse");
    let lookup: HashMap<_, _> = cards
        .iter()
        .map(|c| {
            let id = c.id;
            let matches = c.matching_numbers();
            let copies = (id + 1)..(id + matches as u32 + 1);
            (id, copies)
        })
        .collect();
    let mut card_count = 0;

    for card in cards {
        card_count += copies(card.id, &lookup);
    }

    card_count
}

fn copies(card_id: u32, lookup: &HashMap<u32, Range<u32>>) -> u32 {
    let mut count = 1;

    for copy_id in lookup[&card_id].clone() {
        count += copies(copy_id, &lookup);
    }

    count
}

pub fn part2_try2(input: impl AsRef<str>) -> u32 {
    let input = input.as_ref();
    let (_, cards) = cards(input).expect("cards to parse");
    let mut winnings = vec![1; cards.len()];

    for (original_id, card) in cards.iter().enumerate() {
        for copy_id in card.id..(card.matching_numbers() as u32 + card.id) {
            winnings[copy_id as usize] += winnings[original_id as usize];
        }
    }

    for (i, c) in winnings.iter().enumerate() {
        println!("Card {}: {}", i + 1, c);
    }

    winnings.iter().sum()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
}

impl Card {
    pub fn new(id: u32, winning_numbers: Vec<u32>, numbers: Vec<u32>) -> Self {
        Self {
            id,
            winning_numbers,
            numbers,
        }
    }

    pub fn value(&self) -> u32 {
        let matching = self.matching_numbers();
        let mut result = 0;
        for _ in 0..matching {
            if result == 0 {
                result = 1;
                continue;
            }
            result *= 2;
        }
        result
    }

    pub fn matching_numbers(&self) -> usize {
        self.numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .count()
    }
}

fn numbers(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(tag(" "), preceded(space0, nom_u32))(input)
}

fn card(input: &str) -> IResult<&str, Card> {
    map(
        tuple((
            delimited(tuple((tag("Card"), space1)), nom_u32, tag(": ")),
            numbers,
            preceded(tag(" | "), numbers),
        )),
        |(id, winning_numbers, numbers)| Card::new(id, winning_numbers, numbers),
    )(input)
}

fn cards(input: &str) -> IResult<&str, Vec<Card>> {
    separated_list1(line_ending, card)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"#;

    #[test]
    fn part1_works() {
        let part1 = part1(INPUT);
        assert_eq!(part1, 13);
    }

    #[test]
    fn part2_works() {
        let part2 = part2(INPUT);
        assert_eq!(part2, 30);
    }

    #[test]
    fn part2_try2_works() {
        let part2 = part2_try2(INPUT);
        assert_eq!(part2, 30);
    }

    #[test]
    fn card_works() {
        let expected = Card::new(
            5,
            vec![87, 83, 26, 28, 32],
            vec![88, 30, 70, 12, 93, 22, 82, 36],
        );
        let input = "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36";

        let (_, actual) = card(input).unwrap();

        assert_eq!(actual, expected);
    }
}
