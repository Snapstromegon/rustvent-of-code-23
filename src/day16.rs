use std::{fmt::Display, str::FromStr};

use crate::solution::Solution;

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<usize> {
        let mut map = input.parse::<Map>().unwrap();
        map.calc_visits((0, 0, Direction::Right));
        Some(map.count_visited())
    }

    fn part2(&self, input: &str) -> Option<usize> {
        let map = input.parse::<Map>().unwrap();
        let row_count = map.tiles.len();
        let col_count = map.tiles[0].len();
        let mut max_visits = 0;
        for i in 0..row_count {
            let mut map_l = map.clone();
            map_l.calc_visits((i, 0, Direction::Right));
            if map_l.count_visited() > max_visits {
                max_visits = map_l.count_visited();
            }
            let mut map_r = map.clone();
            map_r.calc_visits((i, col_count - 1, Direction::Left));
            if map_r.count_visited() > max_visits {
                max_visits = map_r.count_visited();
            }
        }
        for i in 0..col_count {
            let mut map_t = map.clone();
            map_t.calc_visits((0, i, Direction::Bottom));
            if map_t.count_visited() > max_visits {
                max_visits = map_t.count_visited();
            }
            let mut map_b = map.clone();
            map_b.calc_visits((row_count - 1, i, Direction::Top));
            if map_b.count_visited() > max_visits {
                max_visits = map_b.count_visited();
            }
        }
        Some(max_visits)
    }
}

#[derive(Debug, Clone)]
struct Map {
    tiles: Vec<Vec<Tile>>,
    visited: Vec<Vec<[bool; 4]>>,
}

impl Map {
    pub fn count_visited(&self) -> usize {
        self.visited
            .iter()
            .map(|row| row.iter().filter(|v| v.iter().any(|b| *b)).count())
            .sum()
    }

    pub fn calc_visits(&mut self, start: (usize, usize, Direction)) {
        let mut stack = vec![start];
        while let Some((row, col, dir)) = stack.pop() {
            if self.visited[row][col][dir.index()] {
                continue;
            }
            self.visited[row][col][dir.index()] = true;
            let tile = self.tiles[row][col];
            let mirrors = dir.mirror(&tile);
            for mirror in mirrors {
                if (row == 0 && mirror == Direction::Top) || (col == 0 && mirror == Direction::Left)
                {
                    continue;
                }
                let (next_row, next_col) = match mirror {
                    Direction::Top => (row - 1, col),
                    Direction::Right => (row, col + 1),
                    Direction::Bottom => (row + 1, col),
                    Direction::Left => (row, col - 1),
                };
                if next_row >= self.tiles.len() || next_col >= self.tiles[0].len() {
                    continue;
                }
                stack.push((next_row, next_col, mirror));
            }
        }
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tiles = vec![];
        for line in s.lines() {
            let mut row = vec![];
            for c in line.chars() {
                row.push(c.into());
            }
            tiles.push(row);
        }
        Ok(Map {
            visited: vec![vec![[false; 4]; tiles[0].len()]; tiles.len()],
            tiles,
        })
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Mirrors:")?;
        for row in &self.tiles {
            for tile in row {
                write!(
                    f,
                    "{}",
                    match tile {
                        Tile::Empty => '.',
                        Tile::MirrorL => '\\',
                        Tile::MirrorR => '/',
                        Tile::SplitH => '-',
                        Tile::SplitV => '|',
                    }
                )?;
            }
            writeln!(f)?;
        }
        writeln!(f, "Visited:")?;
        for row in &self.visited {
            for tile in row {
                write!(
                    f,
                    "{}",
                    match tile.iter().any(|b| *b) {
                        true => '#',
                        false => '.',
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

impl Direction {
    fn index(&self) -> usize {
        match self {
            Direction::Top => 0,
            Direction::Right => 1,
            Direction::Bottom => 2,
            Direction::Left => 3,
        }
    }

    fn mirror(&self, tile: &Tile) -> Vec<Self> {
        match tile {
            Tile::MirrorL => match self {
                Direction::Top => vec![Direction::Left],
                Direction::Right => vec![Direction::Bottom],
                Direction::Bottom => vec![Direction::Right],
                Direction::Left => vec![Direction::Top],
            },
            Tile::MirrorR => match self {
                Direction::Top => vec![Direction::Right],
                Direction::Right => vec![Direction::Top],
                Direction::Bottom => vec![Direction::Left],
                Direction::Left => vec![Direction::Bottom],
            },
            Tile::SplitH => match self {
                Direction::Top | Direction::Bottom => vec![Direction::Left, Direction::Right],
                x => vec![x.clone()],
            },
            Tile::SplitV => match self {
                Direction::Left | Direction::Right => vec![Direction::Top, Direction::Bottom],
                x => vec![x.clone()],
            },
            Tile::Empty => vec![self.clone()],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    MirrorL,
    MirrorR,
    SplitH,
    SplitV,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Tile::Empty,
            '\\' => Tile::MirrorL,
            '/' => Tile::MirrorR,
            '-' => Tile::SplitH,
            '|' => Tile::SplitV,
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
        let input = read_input(16, true, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(46));
    }
    #[test]
    fn test_part1_challenge() {
        let input = read_input(16, false, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(6514));
    }

    #[test]
    fn test_part2_example() {
        let input = read_input(16, true, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(51));
    }
    #[test]
    fn test_part2_challenge() {
        let input = read_input(16, false, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(8089));
    }
}
