use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use crate::solution::Solution;

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<usize> {
        let (path, rest) = input.split_once("\r\n\r\n").unwrap();
        let map: Map = rest.parse().unwrap();
        let path: Vec<Direction> = path.chars().map(|c| c.into()).collect();
        if map.mapping.contains_key("AAA") {
            let mut curr = "AAA";
            for i in 0..100_000_000 {
                curr = map.step(curr, path[i % path.len()]);
                if curr == "ZZZ" {
                    return Some(i + 1);
                }
            }
        } else {
            println!("Map does not contain start key \"AAA\"");
        }
        None
    }

    fn part2(&self, input: &str) -> Option<usize> {
        let (path, rest) = input.split_once("\r\n\r\n").unwrap();
        let map: Map = rest.parse().unwrap();
        let path: Vec<Direction> = path.chars().map(|c| c.into()).collect();
        let starts: Vec<&str> = map
            .mapping
            .keys()
            .filter(|k| k.ends_with('A'))
            .map(|s| &s[..])
            .collect();
        println!("Starts: {starts:?}");
        let mut loops = Vec::new();
        let mut z_pos = HashMap::new();

        for start in starts.clone() {
            let mut loop_detector: HashMap<(String, Direction, usize), usize> = HashMap::new();
            let mut curr = start;
            z_pos.insert(start, Vec::new());
            for i in 0..100_000 {
                if curr.ends_with('Z') {
                    z_pos.get_mut(start).unwrap().push(i);
                }
                let path_pos = i % path.len();
                let dir = path[path_pos];
                let state = (curr.to_string(), dir, path_pos);
                if let Some(loop_start) = loop_detector.get(&state) {
                    loops.push((
                        start,
                        loop_start.clone(),
                        z_pos
                            .get(start)
                            .expect(&("Could not find z_pos for ".to_owned() + start))
                            .clone(),
                        i - loop_start,
                    ));
                    break;
                }
                loop_detector.insert(state, i);
                curr = map.step(curr, dir);
            }
        }
        println!("Loops: {loops:?}");

        // All loops only contain one Z element, which is also the same as the loop length.
        // For this reason you just need to calculate the least common multiple of all these values
        assert!(loops.iter().all(|looped| looped.2.len() == 1 && looped.2[0] == looped.3));
        None
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value.to_ascii_lowercase() {
            'l' => Self::Left,
            'r' => Self::Right,
            _ => unreachable!("Invalid direction"),
        }
    }
}

#[derive(Debug)]
struct Map {
    mapping: HashMap<String, Mapping>,
}

impl<'a> Map {
    pub fn step(&'a self, from: &str, dir: Direction) -> &'a str {
        self.mapping.get(from).unwrap().step(dir)
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            mapping: HashMap::from_iter(
                s.lines()
                    .map(|line| line.parse::<Mapping>().unwrap())
                    .map(|mapping| (mapping.from.clone(), mapping)),
            ),
        })
    }
}

#[derive(Debug)]
struct Mapping {
    from: String,
    left: String,
    right: String,
}

impl<'a> Mapping {
    pub fn step(&'a self, dir: Direction) -> &'a str {
        match dir {
            Direction::Left => &self.left,
            Direction::Right => &self.right,
        }
    }
}

impl FromStr for Mapping {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (from, rest) = s.split_once(" = ").unwrap();
        let (left, right) = rest[1..rest.len() - 1].split_once(", ").unwrap();
        Ok(Self {
            from: from.to_string(),
            left: left.to_string(),
            right: right.to_string(),
        })
    }
}
