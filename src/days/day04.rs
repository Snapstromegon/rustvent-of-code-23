use std::{collections::HashSet, str::FromStr};

use crate::solution::Solution;

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<usize> {
        Some(
            input
                .lines()
                .map(|s| s.parse::<Card>().unwrap())
                .map(|card| (1 << card.winning_count) / 2)
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
            let card_points = cards[i].winning_count;
            for j in 0..card_points {
                if i + j + 1 < card_counts.len() {
                    card_counts[i + j + 1] += card_counts[i];
                }
            }
        }
        Some(card_counts.iter().sum::<usize>())
    }
}

#[derive(Debug, Clone)]
struct Card {
    winning_count: usize,
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, numbers) = s.split_once(':').unwrap();
        let (winning, actual) = numbers.split_once('|').unwrap();
        let winning = winning
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<HashSet<usize>>();
        let winning_count = actual
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .filter(|n| winning.contains(n))
            .count();
        Ok(Card { winning_count })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solution::Solution;
    use crate::utils::read_input;

    #[test]
    fn test_part1_example() {
        let input = read_input(4, true, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(13))
    }
    #[test]
    fn test_part1_challenge() {
        let input = read_input(4, false, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(23235))
    }

    #[test]
    fn test_part2_example() {
        let input = read_input(4, true, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(30))
    }
    #[test]
    fn test_part2_challenge() {
        let input = read_input(4, false, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(5920640))
    }
}
