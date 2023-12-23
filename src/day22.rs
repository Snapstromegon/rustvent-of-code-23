use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use crate::solution::Solution;

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<usize> {
        let mut bricks = input
            .lines()
            .map(|l| l.parse().unwrap())
            .enumerate()
            .map(|(i, mut b): (usize, Brick)| {
                b.id = Some(i);
                b
            })
            .collect::<Vec<Brick>>();
        bricks.sort_by_key(|b| b.min_z());
        let mut supported_blocks = HashMap::new();
        let mut supporting_blocks: HashMap<Id, HashSet<Id>> = HashMap::new();
        for brick in bricks.iter_mut() {
            brick.ground(&mut supported_blocks);
            supporting_blocks.insert(brick.id.unwrap(), brick.supported_by.clone());
        }
        // println!("{:?}", supporting_blocks);
        for (id, supporting) in supporting_blocks.iter() {
            for supporting in supporting.iter() {
                bricks[*supporting].supporting.insert(*id);
            }
        }
        // for brick in bricks.iter() {
        //     println!("{:?} : {:?}", brick, brick.could_be_desintegrated(&bricks));
        // }
        Some(
            bricks
                .iter()
                .filter(|b| b.could_be_desintegrated(&bricks))
                .count(),
        )
    }

    fn part2(&self, _input: &str) -> Option<usize> {
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
    z: usize,
}

impl FromStr for Position {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(',').collect::<Vec<_>>();
        Ok(Position {
            x: parts[0].parse().unwrap(),
            y: parts[1].parse().unwrap(),
            z: parts[2].parse().unwrap(),
        })
    }
}

type Id = usize;

#[derive(Debug)]
struct Brick {
    start: Position,
    end: Position,
    id: Option<Id>,
    supported_by: HashSet<Id>,
    supporting: HashSet<Id>,
}

impl Brick {
    pub fn min_x(&self) -> usize {
        self.start.x.min(self.end.x)
    }

    pub fn max_x(&self) -> usize {
        self.start.x.max(self.end.x)
    }

    pub fn min_y(&self) -> usize {
        self.start.y.min(self.end.y)
    }

    pub fn max_y(&self) -> usize {
        self.start.y.max(self.end.y)
    }

    pub fn min_z(&self) -> usize {
        self.start.z
    }

    pub fn max_z(&self) -> usize {
        self.end.z
    }

    pub fn ground(&mut self, supported_blocks: &mut HashMap<(usize, usize), (usize, Option<Id>)>) {
        let mut max_supported_level = 0;
        let mut supported_by = HashSet::new();

        for x in self.min_x()..=self.max_x() {
            for y in self.min_y()..=self.max_y() {
                let (level, id) = **supported_blocks
                    .get(&(x, y))
                    .get_or_insert(&(0, None));
                if level > max_supported_level {
                    max_supported_level = level;
                    supported_by = HashSet::new();
                }
                if level == max_supported_level {
                    if let Some(id) = id {
                        supported_by.insert(id);
                    }
                }
            }
        }

        let move_down = self.min_z() - 1 - max_supported_level;
        self.supported_by = supported_by;
        self.start.z -= move_down;
        self.end.z -= move_down;
        for x in self.min_x()..=self.max_x() {
            for y in self.min_y()..=self.max_y() {
                supported_blocks.insert((x, y), (self.max_z(), self.id));
            }
        }
    }

    pub fn could_be_desintegrated(&self, bricks: &[Brick]) -> bool {
        self.supporting
            .iter()
            .all(|id| bricks[*id].supported_by.len() != 1)
    }
}

impl FromStr for Brick {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once('~').unwrap();
        let start_pos: Position = start.parse().unwrap();
        let end_pos: Position = end.parse().unwrap();

        // Make sure that end is not lower than start
        if start_pos.z <= end_pos.z {
            Ok(Brick {
                start: start_pos,
                end: end_pos,
                id: None,
                supported_by: HashSet::new(),
                supporting: HashSet::new(),
            })
        } else {
            Ok(Brick {
                start: end_pos,
                end: start_pos,
                id: None,
                supported_by: HashSet::new(),
                supporting: HashSet::new(),
            })
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
        let input = read_input(22, true, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(5));
    }
    #[test]
    fn test_part1_challenge() {
        let input = read_input(22, false, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(401));
    }

    #[test]
    fn test_part2_example() {
        let input = read_input(22, true, 2).unwrap();
        assert_eq!(Day.part2(&input), None);
    }
    #[test]
    fn test_part2_challenge() {
        let input = read_input(22, false, 2).unwrap();
        assert_eq!(Day.part2(&input), None);
    }
}
