use crate::solution::Solution;
use rayon::prelude::*;

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<usize> {
        Some(
            input
                .lines()
                .map(|line| {
                    let mut digits = line.chars().filter(|c| c.is_numeric()).peekable();
                    let first: usize = digits.peek().unwrap().to_digit(10).unwrap() as usize;
                    let last: usize = digits.last().unwrap().to_digit(10).unwrap() as usize;
                    first * 10 + last
                })
                .sum::<usize>(),
        )
    }

    fn part2(&self, input: &str) -> Option<usize> {
        Some(
            input
                .lines()
                .map(|line| {
                    let words = [
                        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight",
                        "nine", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
                    ];

                    let mut earliest_index = usize::MAX;
                    let mut earliest_word = 0;
                    for (index, word) in words.iter().enumerate() {
                        if let Some(found) = line.find(word) {
                            if found <= earliest_index {
                                earliest_index = found;
                                earliest_word = index % 10;
                            }
                        }
                    }

                    let mut latest_index = 0;
                    let mut latest_word = 0;
                    for (index, word) in words.iter().enumerate() {
                        if let Some(found) = line.rfind(word) {
                            if found >= latest_index {
                                latest_index = found;
                                latest_word = index % 10;
                            }
                        }
                    }

                    earliest_word * 10 + latest_word
                })
                .sum(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solution::Solution;
    use crate::utils::read_input;

    #[test]
    fn test_part1_example() {
        let input = read_input(1, true, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(142));
    }
    #[test]
    fn test_part1_challenge() {
        let input = read_input(1, false, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(56049));
    }
    #[test]
    fn test_part2_example() {
        let input = read_input(1, true, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(281))
    }
    #[test]
    fn test_part2_challenge() {
        let input = read_input(1, false, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(54530))
    }
}
