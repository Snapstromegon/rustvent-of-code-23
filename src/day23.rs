use std::{
    collections::HashSet,
    fmt::{Display, Formatter},
    str::FromStr,
};

use crate::solution::Solution;

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<usize> {
        let map = Map::from_str(input).unwrap();
        let from = Position { x: 1, y: 0 };
        let to = Position {
            x: map.tiles[0].len() - 2,
            y: map.tiles.len() - 1,
        };
        let longest_path = map
            .find_longest_path(from, to, Path::possible_directions_block_slopes)
            .unwrap();
        Some(longest_path.len() - 1)
    }

    fn part2(&self, input: &str) -> Option<usize> {
        let map = Map::from_str(input).unwrap();
        let from = Position { x: 1, y: 0 };
        let to = Position {
            x: map.tiles[0].len() - 2,
            y: map.tiles.len() - 1,
        };
        let longest_path = map
            .find_longest_path(from, to, Path::possible_directions)
            .unwrap();
        println!("{}", longest_path);
        Some(longest_path.len() - 1)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Path {
    visited: HashSet<Position>,
    path: Vec<Position>,
}

impl Path {
    pub fn new(start: Position) -> Self {
        let mut visited = HashSet::new();
        visited.insert(start);
        Path {
            visited,
            path: vec![start],
        }
    }

    pub fn len(&self) -> usize {
        self.path.len()
    }

    pub fn current_position(&self) -> Position {
        *self.path.last().unwrap()
    }

    pub fn is_visited(&self, position: Position) -> bool {
        self.visited.contains(&position)
    }

    fn is_slope_blocked(&self, direction: Direction, map: &Map) -> bool {
        let next_position = self.pos_in_direction(direction).unwrap();
        let current_tile = map.get(next_position.x, next_position.y).unwrap();
        if let Tile::Slope(dir) = current_tile {
            dir != direction
        } else {
            false
        }
    }

    fn pos_in_direction(&self, direction: Direction) -> Option<Position> {
        let current_position = self.current_position();
        match direction {
            Direction::Up => {
                if current_position.y > 0 {
                    Some(Position {
                        x: current_position.x,
                        y: current_position.y - 1,
                    })
                } else {
                    None
                }
            }
            Direction::Down => Some(Position {
                x: current_position.x,
                y: current_position.y + 1,
            }),
            Direction::Left => {
                if current_position.x > 0 {
                    Some(Position {
                        x: current_position.x - 1,
                        y: current_position.y,
                    })
                } else {
                    None
                }
            }
            Direction::Right => Some(Position {
                x: current_position.x + 1,
                y: current_position.y,
            }),
        }
    }

    pub fn possible_directions_block_slopes(&self, map: &Map) -> Vec<Direction> {
        let current_position = self.current_position();
        let current_tile = map.get(current_position.x, current_position.y).unwrap();
        if let Tile::Slope(direction) = current_tile {
            if self.is_visited(Position {
                x: current_position.x + 1,
                y: current_position.y,
            }) {
                return vec![];
            } else {
                return vec![direction];
            }
        }
        let possible = self.possible_directions(map);
        possible
            .iter()
            .filter(|d| !self.is_slope_blocked(**d, map))
            .copied()
            .collect()
    }
    pub fn possible_directions(&self, map: &Map) -> Vec<Direction> {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
        .iter()
        .filter(|d| {
            let next_position = self.pos_in_direction(**d);
            next_position.is_some()
                && !self.is_visited(next_position.unwrap())
                && !matches!(
                    map.get(next_position.unwrap().x, next_position.unwrap().y),
                    Some(Tile::Forest)
                )
        })
        .copied()
        .collect::<Vec<Direction>>()
    }

    pub fn move_to(&mut self, direction: Direction) {
        let current_position = self.current_position();
        let new_position = match direction {
            Direction::Up => Position {
                x: current_position.x,
                y: current_position.y - 1,
            },
            Direction::Down => Position {
                x: current_position.x,
                y: current_position.y + 1,
            },
            Direction::Left => Position {
                x: current_position.x - 1,
                y: current_position.y,
            },
            Direction::Right => Position {
                x: current_position.x + 1,
                y: current_position.y,
            },
        };
        self.visited.insert(new_position);
        self.path.push(new_position);
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for line in 0..self.path.iter().map(|p| p.y).max().unwrap() + 1 {
            for column in 0..self.path.iter().map(|p| p.x).max().unwrap() + 1 {
                let position = Position { x: column, y: line };
                if self.path.contains(&position) {
                    write!(f, "X")?;
                } else if self.visited.contains(&position) {
                    write!(f, "O")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Tile {
    Path,
    Forest,
    Slope(Direction),
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Path => '.',
                Tile::Forest => '#',
                Tile::Slope(Direction::Up) => '^',
                Tile::Slope(Direction::Down) => 'v',
                Tile::Slope(Direction::Left) => '<',
                Tile::Slope(Direction::Right) => '>',
            }
        )
    }
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Tile::Path,
            '#' => Tile::Forest,
            '^' => Tile::Slope(Direction::Up),
            'v' => Tile::Slope(Direction::Down),
            '<' => Tile::Slope(Direction::Left),
            '>' => Tile::Slope(Direction::Right),
            _ => panic!("Invalid tile: {}", c),
        }
    }
}

#[derive(Debug, Clone)]
struct Map {
    tiles: Vec<Vec<Tile>>,
}

impl Map {
    fn get(&self, x: usize, y: usize) -> Option<Tile> {
        self.tiles.get(y).and_then(|line| line.get(x)).copied()
    }

    pub fn find_longest_path(
        &self,
        from: Position,
        to: Position,
        direction_finder: fn(&Path, &Map) -> Vec<Direction>,
    ) -> Option<Box<Path>> {
        let mut queue = Vec::new();
        queue.push(Path::new(from));
        let mut longest: Option<Box<Path>> = None;
        while let Some(path) = queue.pop() {
            let current_position = path.current_position();
            if current_position == to {
                if let Some(longest) = longest.as_mut() {
                    if path.len() > longest.len() {
                        *longest = Box::new(path.clone());
                        print!("{esc}c", esc = 27 as char);
                        println!(
                            "Found path of length {:?}",
                            longest.len()
                        );
                        println!("{longest}");
                    }
                } else {
                    longest = Some(Box::new(path.clone()));
                }
            } else {
                for direction in direction_finder(&path, self) {
                    let mut new_path = path.clone();
                    new_path.move_to(direction);
                    queue.push(new_path);
                }
            }
        }

        longest
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles = s
            .lines()
            .map(|line| line.chars().map(Tile::from).collect())
            .collect();
        Ok(Map { tiles })
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for line in &self.tiles {
            for tile in line {
                write!(f, "{tile}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solution::Solution;
    use crate::utils::read_input;

    #[test]
    fn test_part1_example() {
        let input = read_input(23, true, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(94));
    }
    #[test]
    fn test_part1_challenge() {
        let input = read_input(23, false, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(2094));
    }

    #[test]
    fn test_part2_example() {
        let input = read_input(23, true, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(154));
    }
    #[test]
    fn test_part2_challenge() {
        let input = read_input(23, false, 2).unwrap();
        assert_eq!(Day.part2(&input), None);
    }
}
