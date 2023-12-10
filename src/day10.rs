use std::{str::FromStr, fmt::Display};

use crate::solution::Solution;

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<usize> {
        let map: Map = input.parse().unwrap();
        let length = map.loop_length();
        Some(length / 2)
    }

    fn part2(&self, input: &str) -> Option<usize> {
        let map: Map = input.parse().unwrap();
        let extract = map.extract_loop();
        println!("Extracted loop: \n{}", extract);
        None
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Map {
    map: Vec<Vec<Pipe>>,
    start: (usize, usize),
}

impl Map {
    pub fn loop_length(&self) -> usize {
        let mut current = self.start;
        let mut dir: Direction = [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
        .iter()
        .find(|dir| self.can_move(current, **dir))
        .unwrap()
        .clone();
        let mut length = 0;
        loop {
            length += 1;
            current = self.pos_in_dir(current, dir);
            if current == self.start {
                break;
            }
            dir = self.map[current.0][current.1].other_dir(dir);
        }
        return length;
    }

    pub fn extract_loop(&self) -> Self {
        let mut current = self.start;
        let mut dir: Direction = [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
        .iter()
        .find(|dir| self.can_move(current, **dir))
        .unwrap()
        .clone();
        let mut extract = Map {
            map: vec![vec![Pipe::Empty; self.map[0].len()]; self.map.len()],
            start: self.start,
        };
        loop {
            extract.map[current.0][current.1] = self.map[current.0][current.1];
            current = self.pos_in_dir(current, dir);
            if current == self.start {
                break;
            }
            dir = self.map[current.0][current.1].other_dir(dir);
        }
        extract
    }

    pub fn pos_in_dir(&self, pos: (usize, usize), dir: Direction) -> (usize, usize) {
        match dir {
            Direction::Up => (pos.0 - 1, pos.1),
            Direction::Down => (pos.0 + 1, pos.1),
            Direction::Left => (pos.0, pos.1 - 1),
            Direction::Right => (pos.0, pos.1 + 1),
        }
    }

    pub fn pos_dir_possible(&self, pos: (usize, usize), dir: Direction) -> bool {
        match dir {
            Direction::Up => pos.0 > 0,
            Direction::Down => pos.0 < self.map.len() - 1,
            Direction::Left => pos.1 > 0,
            Direction::Right => pos.1 < self.map[0].len() - 1,
        }
    }

    pub fn can_move(&self, pos: (usize, usize), dir: Direction) -> bool {
        if !self.pos_dir_possible(pos, dir) {
            return false;
        }
        let next = self.pos_in_dir(pos, dir);
        let next_pipe = self.map[next.0][next.1].clone();
        // println!("Can move {:?} from {:?} to {:?} ({:?})", dir, pos, next, next_pipe);
        match next_pipe {
            Pipe::Empty => false,
            _ => match (dir, next_pipe) {
                (Direction::Up, Pipe::CornerBottomLeft) => true,
                (Direction::Up, Pipe::CornerBottomRight) => true,
                (Direction::Down, Pipe::CornerTopLeft) => true,
                (Direction::Down, Pipe::CornerTopRight) => true,
                (Direction::Left, Pipe::CornerTopRight) => true,
                (Direction::Left, Pipe::CornerBottomRight) => true,
                (Direction::Right, Pipe::CornerTopLeft) => true,
                (Direction::Right, Pipe::CornerBottomLeft) => true,
                (Direction::Up, Pipe::Vertical) => true,
                (Direction::Down, Pipe::Vertical) => true,
                (Direction::Left, Pipe::Horizontal) => true,
                (Direction::Right, Pipe::Horizontal) => true,
                _ => false,
            },
        }
    }
}

impl FromStr for Map {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = (0, 0);
        let map = s
            .lines()
            .enumerate()
            .map(|(row, l)| {
                l.chars()
                    .enumerate()
                    .map(|(col, c)| {
                        let p = Pipe::from(c);
                        if p == Pipe::Start {
                            start = (row, col);
                        }
                        p
                    })
                    .collect::<Vec<Pipe>>()
            })
            .collect::<Vec<Vec<Pipe>>>();
        Ok(Map { map, start })
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.map {
            for pipe in row {
                write!(f, "{}", pipe)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Pipe {
    Empty,
    Start,
    Horizontal,
    Vertical,
    CornerTopRight,
    CornerTopLeft,
    CornerBottomRight,
    CornerBottomLeft,
}

impl Pipe {
    pub fn other_dir(&self, dir: Direction) -> Direction {
        match (self, dir) {
            (Pipe::CornerBottomLeft, Direction::Up) => Direction::Left,
            (Pipe::CornerBottomLeft, Direction::Right) => Direction::Down,
            (Pipe::CornerBottomRight, Direction::Up) => Direction::Right,
            (Pipe::CornerBottomRight, Direction::Left) => Direction::Down,
            (Pipe::CornerTopLeft, Direction::Down) => Direction::Left,
            (Pipe::CornerTopLeft, Direction::Right) => Direction::Up,
            (Pipe::CornerTopRight, Direction::Down) => Direction::Right,
            (Pipe::CornerTopRight, Direction::Left) => Direction::Up,
            (Pipe::Horizontal, Direction::Left) => Direction::Left,
            (Pipe::Horizontal, Direction::Right) => Direction::Right,
            (Pipe::Vertical, Direction::Up) => Direction::Up,
            (Pipe::Vertical, Direction::Down) => Direction::Down,
            _ => panic!("Invalid pipe/dir combination: {:?} {:?}", self, dir),
        }
    }
}

impl From<char> for Pipe {
    fn from(c: char) -> Self {
        match c {
            'S' => Pipe::Start,
            '-' => Pipe::Horizontal,
            '|' => Pipe::Vertical,
            'L' => Pipe::CornerTopRight,
            'J' => Pipe::CornerTopLeft,
            'F' => Pipe::CornerBottomRight,
            '7' => Pipe::CornerBottomLeft,
            '.' => Pipe::Empty,
            _ => panic!("Invalid pipe char: {}", c),
        }
    }
}

impl Display for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Pipe::Empty => '.',
            Pipe::Start => 'S',
            Pipe::Horizontal => '-',
            Pipe::Vertical => '|',
            Pipe::CornerTopRight => 'L',
            Pipe::CornerTopLeft => 'J',
            Pipe::CornerBottomRight => 'F',
            Pipe::CornerBottomLeft => '7',
        };
        write!(f, "{}", c)
    }
}
