use std::str::FromStr;

use crate::solution::Solution;

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<usize> {
        Some(
            input
                .lines()
                .map(|s| s.parse::<Card>().unwrap())
                .map(|card| 2usize.pow(card.number_of_winning() as u32) / 2)
                .sum(),
        )
    }

    fn part2(&self, input: &str) -> Option<usize> {
        let cards = input
            .lines()
            .map(|s| s.parse::<Card>().unwrap())
            .collect::<Vec<Card>>();
        let mut card_counts: Vec<usize> = (0..cards.len()).map(|_| 1).collect();
        for i in 0..cards.len() {
            let card_points = cards[i].number_of_winning();
            for j in 0..card_points {
                if i + j + 1 < card_counts.len() {
                    card_counts[i+j+1] += card_counts[i];
                }
            }
        }
        Some(card_counts.iter().sum::<usize>())
    }
}

#[derive(Debug, Clone)]
struct Card {
    winning: Vec<usize>,
    actual: Vec<usize>,
}

impl Card {
    pub fn number_of_winning(&self) -> usize {
        self.actual
            .iter()
            .filter(|n| self.winning.contains(n))
            .count()
    }
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, numbers) = s.split_once(':').unwrap();
        let (winning, actual) = numbers.split_once('|').unwrap();
        let winning = winning
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let actual = actual
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        Ok(Card {
            winning,
            actual,
        })
    }
}
