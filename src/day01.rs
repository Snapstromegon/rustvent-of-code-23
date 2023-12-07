use crate::solution::Solution;

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<usize> {
        Some(
            input
                .lines()
                .map(|line| {
                    let digits = line.chars().filter(|c| c.is_numeric());
                    let first = digits.clone().next().unwrap();
                    let last = digits.last().unwrap();
                    let number = first.to_string() + &last.to_string();
                    number.parse::<usize>().unwrap()
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

                    (earliest_word.to_string() + &latest_word.to_string())
                        .parse::<usize>()
                        .unwrap()
                })
                .sum(),
        )
    }
}
