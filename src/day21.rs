use std::{fmt::Display, str::FromStr};

use crate::solution::Solution;

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<usize> {
        let map: Map = input.parse().unwrap();
        let distance_map = map.calc_distance_map(64);
        let reachable_plots = distance_map
            .iter()
            .flatten()
            .filter(|d| d.is_some())
            .filter(|d| d.unwrap() % 2 == 0)
            .count();
        Some(reachable_plots)
    }

    fn part2(&self, _input: &str) -> Option<usize> {
        None
    }
}

fn _print_distances(distance_map: &Vec<Vec<Option<usize>>>) {
    for row in distance_map {
        for distance in row {
            print!(
                "{} ",
                match distance {
                    Some(d) => format!("{:02}", d),
                    None => "XX".to_string(),
                }
            );
        }
        println!();
    }
}

#[derive(Debug)]
struct Map {
    tiles: Vec<Vec<Tile>>,
    start: (usize, usize),
}

impl Map {
    fn get(&self, coords: (usize, usize)) -> Option<Tile> {
        self.tiles
            .get(coords.1 % self.tiles.len())
            .and_then(|row| row.get(coords.0 % self.tiles[0].len()))
            .copied()
    }

    fn normalize_coordinates(&self, coords: (usize, usize)) -> (usize, usize) {
        (coords.0 % self.tiles[0].len(), coords.1 % self.tiles.len())
    }

    fn get_possible_neighbors(&self, coords: (usize, usize)) -> Vec<(usize, usize)> {
        let (mut x, mut y) = coords;
        let mut neighbors = Vec::new();
        if y == 0 {
            y += self.tiles.len();
        }
        if x == 0 {
            x += self.tiles[0].len();
        }
        if let Some(Tile::Plot) = self.get((x, y - 1)) {
            neighbors.push((x, y - 1));
        }
        if let Some(Tile::Plot) = self.get((x, y + 1)) {
            neighbors.push((x, y + 1));
        }
        if let Some(Tile::Plot) = self.get((x - 1, y)) {
            neighbors.push((x - 1, y));
        }
        if let Some(Tile::Plot) = self.get((x + 1, y)) {
            neighbors.push((x + 1, y));
        }
        neighbors
    }

    fn calc_distance_map(&self, max_distance: usize) -> Vec<Vec<Option<usize>>> {
        let mut distance_map = vec![vec![None; self.tiles[0].len()]; self.tiles.len()];
        let mut queue = Vec::new();
        queue.push((self.start.0, self.start.1, 0));
        while let Some((x, y, distance)) = queue.pop() {
            let (nx, ny) = self.normalize_coordinates((x, y));
            if distance_map[ny][nx].is_none() || Some(distance) < distance_map[ny][nx] {
                distance_map[ny][nx] = Some(distance);
                if distance < max_distance {
                    for (nx, ny) in self.get_possible_neighbors((x, y)) {
                        queue.push((nx, ny, distance + 1));
                    }
                }
            }
        }
        distance_map
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles: Vec<Vec<Tile>> = s
            .lines()
            .map(|line| line.chars().map(Tile::from).collect())
            .collect();
        let start = tiles
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter().enumerate().find_map(|(x, tile)| {
                    if *tile == Tile::Start {
                        Some((x, y))
                    } else {
                        None
                    }
                })
            })
            .unwrap();
        Ok(Map { tiles, start })
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.tiles {
            for tile in row {
                write!(
                    f,
                    "{}",
                    match tile {
                        Tile::Start => 'S',
                        Tile::Plot => '.',
                        Tile::Rock => '#',
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Start,
    Plot,
    Rock,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Plot,
            '#' => Self::Rock,
            'S' => Self::Start,
            _ => panic!("Invalid tile: {}", c),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solution::Solution;
    use crate::utils::read_input;

    #[test]
    fn test_part1_example() {
        let input = read_input(21, true, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(42));
    }
    #[test]
    fn test_part1_challenge() {
        let input = read_input(21, false, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(3751));
    }

    #[test]
    fn test_part2_example() {
        let input = read_input(21, true, 2).unwrap();
        assert_eq!(Day.part2(&input), None);
    }
    #[test]
    fn test_part2_challenge() {
        let input = read_input(21, false, 2).unwrap();
        assert_eq!(Day.part2(&input), None);
    }
}
