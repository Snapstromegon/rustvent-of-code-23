use crate::solution::Solution;

pub struct Day;

impl Solution for Day {
    /// ```
    /// # use rustvent::utils::read_input;
    /// # use rustvent::solution::Solution;
    /// # use rustvent::day06::Day;
    /// let input = read_input(6, true, 1).unwrap();
    /// assert_eq!(Day.part1(&input), Some(288))
    /// ```
    ///
    /// ```
    /// # use rustvent::utils::read_input;
    /// # use rustvent::solution::Solution;
    /// # use rustvent::day06::Day;
    /// let input = read_input(6, false, 1).unwrap();
    /// assert_eq!(Day.part1(&input), Some(128700))
    /// ```
    fn part1(&self, input: &str) -> Option<usize> {
        let races = parse_input(input);
        /*
           s_a(t) = t * (a-t)
           r = -t^2 + a*t
           0 = -t^2 + a*t - r
           0 = t^2 - a*t + r
        */
        Some(
            races
                .iter()
                .map(race_win_range)
                .map(|(min, max)| max - min + 1)
                .product::<i64>() as usize,
        )
    }

    /// ```
    /// # use rustvent::utils::read_input;
    /// # use rustvent::solution::Solution;
    /// # use rustvent::day06::Day;
    /// let input = read_input(6, true, 2).unwrap();
    /// assert_eq!(Day.part2(&input), Some(71503))
    /// ```
    ///
    /// ```
    /// # use rustvent::utils::read_input;
    /// # use rustvent::solution::Solution;
    /// # use rustvent::day06::Day;
    /// let input = read_input(6, false, 2).unwrap();
    /// assert_eq!(Day.part2(&input), Some(39594072))
    /// ```
    fn part2(&self, input: &str) -> Option<usize> {
        let lines: Vec<i64> = input
            .lines()
            .map(|line| line.split_once(':').unwrap().1.trim().replace(' ', ""))
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        let race = (lines[0], lines[1]);
        let win_range = race_win_range(&race);
        Some((win_range.1 - win_range.0 + 1) as usize)
    }
}

fn race_win_range((time, distance): &(i64, i64)) -> (i64, i64) {
    pq_formula(*time, *distance)
}

fn pq_formula(p: i64, q: i64) -> (i64, i64) {
    let p = p as f64;
    let q = q as f64;
    let half_p = p / 2.;
    let sqrt = ((half_p * half_p) - q).sqrt();
    let min = 0. - half_p - sqrt;
    let max = 0. - half_p + sqrt;
    ((min.floor() + 1.) as i64, (max.ceil() - 1.) as i64)
}

fn parse_input(input: &str) -> Vec<(i64, i64)> {
    let mut iters = input.lines().map(|line| {
        line.split_whitespace()
            .skip(1)
            .map(|s| s.parse::<i64>().unwrap())
    });
    let times = iters.next().unwrap();
    let distances = iters.next().unwrap();
    times.zip(distances).collect()
}
