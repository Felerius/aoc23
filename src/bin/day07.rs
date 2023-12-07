use std::{
    array,
    cmp::{Ordering, Reverse},
    io::{self, BufRead},
    mem,
};

use anyhow::{Context, Ok, Result};
use itertools::Itertools;

const CARDS: &[u8] = b"23456789TJQKA";
const NUM_CARDS: usize = CARDS.len();
const JOKER: u8 = 9;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn detect(hand: [u8; 5], use_jokers: bool) -> Self {
        let mut card_counts = hand
            .iter()
            .copied()
            .fold([0; NUM_CARDS], |mut counts, card| {
                counts[card as usize] += 1;
                counts
            });
        let joker_count = if use_jokers {
            mem::replace(&mut card_counts[JOKER as usize], 0)
        } else {
            0
        };
        let counts: Vec<_> = card_counts
            .iter()
            .copied()
            .sorted_by_key(|c| Reverse(*c))
            .filter(|&c| c > 0)
            .collect();

        if joker_count == 5 || counts[0] + joker_count == 5 {
            Self::FiveOfAKind
        } else if counts[0] + joker_count == 4 {
            Self::FourOfAKind
        } else if counts[0] <= 3 && counts[1] <= 2 && counts[0] + counts[1] + joker_count == 5 {
            Self::FullHouse
        } else if counts[0] + joker_count == 3 {
            Self::ThreeOfAKind
        } else if counts[0] + counts[1] + joker_count == 4 {
            Self::TwoPairs
        } else if counts[0] + joker_count == 2 {
            Self::OnePair
        } else {
            Self::HighCard
        }
    }
}

fn main() -> Result<()> {
    let hands = io::stdin()
        .lock()
        .lines()
        .map(|line| {
            let line = line?;
            let (hand, bid) = line.split_once(' ').context("invalid input")?;
            let hand: [_; 5] = array::from_fn(|i| {
                let card = hand.as_bytes()[i];
                CARDS.iter().position(|&c| c == card).unwrap() as u8
            });
            let bid = bid.parse::<usize>()?;
            Ok((hand, bid))
        })
        .collect::<Result<Vec<_>>>()?;

    let part1 = hands
        .iter()
        .copied()
        .map(|(hand, bid)| {
            let hand_type = HandType::detect(hand, false);
            (hand_type, hand, bid)
        })
        .sorted_unstable()
        .enumerate()
        .map(|(i, (_, _, bid))| (i + 1) * bid as usize)
        .sum::<usize>();

    let part2 = hands
        .iter()
        .copied()
        .map(|(mut hand, bid)| {
            let hand_type = HandType::detect(hand, true);
            for card in &mut hand {
                *card = match (*card).cmp(&JOKER) {
                    Ordering::Less => *card + 1,
                    Ordering::Equal => 0,
                    Ordering::Greater => *card,
                };
            }
            (hand_type, hand, bid)
        })
        .sorted_unstable()
        .enumerate()
        .map(|(i, (_, _, bid))| (i + 1) * bid as usize)
        .sum::<usize>();

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
    Ok(())
}
