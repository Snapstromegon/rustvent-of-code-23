use crate::solution::Solution;

pub struct Day;

impl Solution for Day {
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
                .fold(1, |a, b| a * b) as usize,
        )
    }

    fn part2(&self, input: &str) -> Option<usize> {
        let lines: Vec<i64> = input
            .lines()
            .map(|line| line.split_once(':').unwrap().1.trim().replace(" ", ""))
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        let race = (lines[0] as i64, lines[1] as i64);
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
