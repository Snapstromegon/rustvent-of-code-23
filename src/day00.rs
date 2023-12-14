use crate::solution::Solution;

pub struct Day;

impl Solution for Day {
    /// ```
    /// # use rustvent::utils::read_input;
    /// # use rustvent::solution::Solution;
    /// # use rustvent::day00::Day;
    /// let input = read_input(0, true, 1).unwrap();
    /// assert_eq!(Day.part1(&input), None)
    /// ```
    ///
    /// ```
    /// # use rustvent::utils::read_input;
    /// # use rustvent::solution::Solution;
    /// # use rustvent::day00::Day;
    /// let input = read_input(0, false, 1).unwrap();
    /// assert_eq!(Day.part1(&input), None)
    /// ```
    fn part1(&self, _input: &str) -> Option<usize> {
        None
    }

    /// ```
    /// # use rustvent::utils::read_input;
    /// # use rustvent::solution::Solution;
    /// # use rustvent::day00::Day;
    /// let input = read_input(0, true, 2).unwrap();
    /// assert_eq!(Day.part2(&input), None)
    /// ```
    ///
    /// ```
    /// # use rustvent::utils::read_input;
    /// # use rustvent::solution::Solution;
    /// # use rustvent::day00::Day;
    /// let input = read_input(0, false, 2).unwrap();
    /// assert_eq!(Day.part2(&input), None)
    /// ```
    fn part2(&self, _input: &str) -> Option<usize> {
        None
    }
}
