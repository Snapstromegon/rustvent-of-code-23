use std::{collections::HashMap, str::FromStr};

use crate::solution::Solution;

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<usize> {
        let (path, rest) = input
            .split_once("\r\n\r\n")
            .or_else(|| input.split_once("\n\n"))
            .unwrap();
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
        let (path, rest) = input
            .split_once("\r\n\r\n")
            .or_else(|| input.split_once("\n\n"))
            .unwrap();
        let map: Map = rest.parse().unwrap();
        let path: Vec<Direction> = path.chars().map(|c| c.into()).collect();
        let starts: Vec<&str> = map
            .mapping
            .keys()
            .filter(|k| k.ends_with('A'))
            .map(|s| &s[..])
            .collect();
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
                        *loop_start,
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

        // All loops only contain one Z element, which is also the same as the loop length.
        // For this reason you just need to calculate the least common multiple of all these values
        assert!(loops
            .iter()
            .all(|looped| looped.2.iter().any(|l| l.eq(&looped.3))));
        let loop_lengths: Vec<usize> = loops.iter().map(|looped| looped.3).collect();
        let lcm = loop_lengths.iter().fold(1, |acc, &x| lcm(acc, x));
        Some(lcm)
    }
}

pub fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

pub fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_input;

    #[test]
    fn test_part1_example() {
        let input = read_input(8, true, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(6));
    }
    #[test]
    fn test_part1_challenge() {
        let input = read_input(8, false, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(22357));
    }
    #[test]
    fn test_part2_example() {
        let input = read_input(8, true, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(6));
    }

    #[test]
    fn test_part2_challenge() {
        let input = read_input(8, false, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(10371555451871));
    }
}
