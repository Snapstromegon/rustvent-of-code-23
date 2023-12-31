use std::{collections::HashMap, str::FromStr};

use crate::solution::Solution;

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<usize> {
        let mut hands = input
            .lines()
            .map(|line| line.parse::<Hand>().unwrap())
            .collect::<Vec<Hand>>();
        hands.sort();
        let winnings = hands
            .iter()
            .enumerate()
            .map(|(i, hand)| hand.bid * (i + 1))
            .sum::<usize>();
        Some(winnings)
    }

    fn part2(&self, input: &str) -> Option<usize> {
        let mut hands = input
            .lines()
            .map(|line| line.parse::<Hand>().unwrap())
            .map(|hand| hand.jokerize())
            .collect::<Vec<Hand>>();
        hands.sort();
        let winnings = hands
            .iter()
            .enumerate()
            .map(|(i, hand)| (hand, hand.bid * (i + 1)));
        Some(winnings.map(|(_, bid)| bid).sum::<usize>())
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<Card>,
    hand_type: Type,
    bid: usize,
}

impl Hand {
    fn new(cards: Vec<Card>, bid: usize) -> Self {
        let hand_type = Self::get_hand_type(&cards);
        Self {
            cards,
            hand_type,
            bid,
        }
    }

    fn jokerize(&self) -> Self {
        let jokerized: Vec<Card> = self.cards.iter().map(|c| c.jokerize()).collect();
        let new_hand_type = Self::get_hand_type(&jokerized);
        Self {
            cards: jokerized,
            hand_type: new_hand_type,
            bid: self.bid,
        }
    }

    fn get_hand_type(cards: &[Card]) -> Type {
        let mut counts = HashMap::new();
        for card in cards {
            *counts.entry(card).or_insert(0) += 1;
        }
        let jokers = counts.remove(&Card::Joker).unwrap_or(0);
        if jokers == 5 {
            return Type::FiveOfAKind;
        }
        let mut counts = counts.into_iter().collect::<Vec<_>>();
        counts.sort_by(|a, b| b.1.cmp(&a.1));
        let (_card, count) = counts[0];
        if jokers > 0 {
            let count = count + jokers;
            match count {
                5 => Type::FiveOfAKind,
                4 => Type::FourOfAKind,
                3 => {
                    if counts.len() == 2 {
                        Type::FullHouse
                    } else {
                        Type::ThreeOfAKind
                    }
                }
                2 => Type::OnePair,
                1 => Type::HighCard,
                _ => panic!("Invalid card count"),
            }
        } else {
            match count {
                5 => Type::FiveOfAKind,
                4 => Type::FourOfAKind,
                3 => {
                    if counts.len() == 2 {
                        Type::FullHouse
                    } else {
                        Type::ThreeOfAKind
                    }
                }
                2 => {
                    if counts.len() == 3 {
                        Type::TwoPair
                    } else {
                        Type::OnePair
                    }
                }
                1 => Type::HighCard,
                _ => panic!("Invalid card count"),
            }
        }
    }
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s.split_once(' ').unwrap();
        let cards = cards
            .chars()
            .map(|s| s.into())
            .collect::<Vec<Card>>();
        let bid = bid.parse::<usize>().unwrap();
        Ok(Self::new(cards, bid))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            std::cmp::Ordering::Equal => {
                for i in 0..self.cards.len() {
                    let compare = self.cards[i].cmp(&other.cards[i]);
                    if compare != std::cmp::Ordering::Equal {
                        return compare;
                    }
                }
                std::cmp::Ordering::Equal
            }
            x => x,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
enum Card {
    Joker,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    CT,
    CJ,
    CQ,
    CK,
    CA,
}

impl Card {
    fn jokerize(self) -> Self {
        match self {
            Self::CJ => Self::Joker,
            x => x,
        }
    }
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            '2' => Self::C2,
            '3' => Self::C3,
            '4' => Self::C4,
            '5' => Self::C5,
            '6' => Self::C6,
            '7' => Self::C7,
            '8' => Self::C8,
            '9' => Self::C9,
            'T' => Self::CT,
            'J' => Self::CJ,
            'Q' => Self::CQ,
            'K' => Self::CK,
            'A' => Self::CA,
            _ => unreachable!("Invalid card"),
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solution::Solution;
    use crate::utils::read_input;

    #[test]
    fn test_part1_example() {
        let input = read_input(7, true, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(6440))
    }
    #[test]
    fn test_part1_challenge() {
        let input = read_input(7, false, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(253603890))
    }

    #[test]
    fn test_part2_example() {
        let input = read_input(7, true, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(5905))
    }
    #[test]
    fn test_part2_challenge() {
        let input = read_input(7, false, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(253630098))
    }
}
