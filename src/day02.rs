use std::str::FromStr;

use crate::solution::Solution;

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<usize> {
        let games = input.lines().map(|s| s.parse::<Game>().unwrap());
        Some(games.filter(|game| game.is_possible((12, 13, 14))).map(|game| game.id).sum())
    }

    fn part2(&self, input: &str) -> Option<usize> {
        let games = input.lines().map(|s| s.parse::<Game>().unwrap());
        Some(games.map(|game| game.get_power()).sum())
    }
}

#[derive(Debug)]
struct Game {
    id: usize,
    draws: Vec<(usize, usize, usize)>,
}

impl Game {
    pub fn is_possible(&self, max: (usize, usize, usize)) -> bool {
        for draw in &self.draws {
            if draw.0 > max.0 || draw.1 > max.1 || draw.2 > max.2 {
                return false;
            }
        }
        true
    }

    pub fn get_power(&self) -> usize {
        let mut max = (0, 0, 0);
        for draw in &self.draws {
            if draw.0 > max.0 {
                max.0 = draw.0;
            }
            if draw.1 > max.1 {
                max.1 = draw.1;
            }
            if draw.2 > max.2 {
                max.2 = draw.2;
            }
        }
        max.0 * max.1 * max.2
    }
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, bag_draws) = s.split_once(':').unwrap();
        let id = id.split_once(' ').unwrap().1.parse::<usize>().unwrap();
        let draws = bag_draws
            .split(';')
            .map(|s| s.trim())
            .map(|s| s.split(',').map(|s| s.trim()))
            .map(|pulls| {
                let mut draw = (0, 0, 0);
                for pull in pulls {
                    let (count, color) = pull.split_once(' ').unwrap();
                    match color {
                        "red" => draw.0 = count.parse::<usize>().unwrap(),
                        "green" => draw.1 = count.parse::<usize>().unwrap(),
                        "blue" => draw.2 = count.parse::<usize>().unwrap(),
                        _ => panic!("Unknown color {}", color),
                    }
                }
                draw
            })
            .collect();
        Ok(Game { id, draws })
    }
}
