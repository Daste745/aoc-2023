#![allow(dead_code)]
#![allow(unused_variables)]

use core::panic;
use std::{collections::HashMap, fs::read_to_string, str::Lines};

fn main() {
    println!("==== Part 1 output ====");
    part_1(read_to_string("day07/part1.input.txt").unwrap().lines());
    //     println!("\n==== Part 2 output ====");
    //     part_2(read_to_string("day07/part2.input.txt").unwrap().lines());
}

#[derive(Debug, PartialEq, PartialOrd)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl From<&str> for HandType {
    fn from(value: &str) -> Self {
        let mut card_counts: HashMap<char, usize> = HashMap::new();
        value
            .chars()
            .for_each(|c| *card_counts.entry(c).or_default() += 1);

        if card_counts.len() == 1 {
            return Self::FiveOfAKind;
        }
        if card_counts.values().any(|v| *v == 4) {
            return Self::FourOfAKind;
        }
        if card_counts.values().any(|v| *v == 3) && card_counts.values().any(|v| *v == 2) {
            return Self::FullHouse;
        }
        if card_counts.values().any(|v| *v == 3) {
            return Self::ThreeOfAKind;
        }
        if card_counts.values().filter(|v| **v == 2).count() == 2 {
            return Self::TwoPair;
        }
        if card_counts.values().any(|v| *v == 2) && card_counts.len() == 4 {
            return Self::OnePair;
        }
        if card_counts.len() == 5 {
            return Self::HighCard;
        }

        panic!("No HandType match for {value}");
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Self::A,
            'K' => Self::K,
            'Q' => Self::Q,
            'J' => Self::J,
            'T' => Self::T,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            _ => panic!(),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Hand {
    type_: HandType,
    cards: Vec<Card>,
    bid: usize,
}

impl Hand {
    fn from_str(hand_str: &str, bid: usize) -> Hand {
        Self {
            type_: HandType::from(hand_str),
            cards: hand_str.chars().map(|c| Card::from(c)).collect(),
            bid,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.type_.partial_cmp(&other.type_).unwrap() {
            std::cmp::Ordering::Less => Some(std::cmp::Ordering::Less),
            std::cmp::Ordering::Greater => Some(std::cmp::Ordering::Greater),
            std::cmp::Ordering::Equal => {
                let pairs = self.cards.iter().zip(&other.cards);
                for (lhs, rhs) in pairs {
                    let res = match lhs.partial_cmp(&rhs).unwrap() {
                        std::cmp::Ordering::Less => Some(std::cmp::Ordering::Less),
                        std::cmp::Ordering::Greater => Some(std::cmp::Ordering::Greater),
                        std::cmp::Ordering::Equal => None,
                    };
                    if res != None {
                        return res;
                    }
                }
                Some(std::cmp::Ordering::Equal)
            }
        }
    }
}

fn part_1(lines: Lines) {
    let mut hands: Vec<Hand> = Vec::new();

    for line in lines {
        let mut parts = line.split_whitespace();
        let hand_str = parts.next().unwrap();
        let bid = parts.next().unwrap().parse::<usize>().unwrap();
        let hand = Hand::from_str(hand_str, bid);
        hands.push(hand);
        // dbg!(hand);
    }

    // Sort weakest to strongest
    hands.sort_by(|a, b| b.partial_cmp(&a).unwrap());
    // dbg!(hands);

    let total = hands
        .iter()
        .enumerate()
        .map(|(idx, hand)| (idx + 1) * hand.bid)
        .sum::<usize>();
    println!("total: {total}");
}

fn part_2(lines: Lines) {}
