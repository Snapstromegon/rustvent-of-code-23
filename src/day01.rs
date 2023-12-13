use crate::solution::Solution;

pub struct Day;

impl Solution for Day {
    /// ```
    /// # use rustvent::utils::read_input;
    /// # use rustvent::solution::Solution;
    /// # use rustvent::day01::Day;
    /// let input = read_input(1, true, 1).unwrap();
    /// assert_eq!(Day.part1(&input), Some(142))
    /// ```
    ///
    /// ```
    /// # use rustvent::utils::read_input;
    /// # use rustvent::solution::Solution;
    /// # use rustvent::day01::Day;
    /// let input = read_input(1, false, 1).unwrap();
    /// assert_eq!(Day.part1(&input), Some(56049))
    /// ```
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

    /// ```
    /// # use rustvent::utils::read_input;
    /// # use rustvent::solution::Solution;
    /// # use rustvent::day01::Day;
    /// let input = read_input(1, true, 2).unwrap();
    /// assert_eq!(Day.part2(&input), Some(281))
    /// ```
    ///
    /// ```
    /// # use rustvent::utils::read_input;
    /// # use rustvent::solution::Solution;
    /// # use rustvent::day01::Day;
    /// let input = read_input(1, false, 2).unwrap();
    /// assert_eq!(Day.part2(&input), Some(54530))
    /// ```
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
