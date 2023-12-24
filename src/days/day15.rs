use crate::solution::Solution;

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<usize> {
        Some(input.split(',').map(xmas_hash).sum())
    }

    fn part2(&self, input: &str) -> Option<usize> {
        let mut boxes: Vec<LensBox> = (0..=255).map(|_| LensBox { lenses: vec![] }).collect();
        input
            .split(',')
            .map(|s| s.into())
            .for_each(|op: Operation| {
                boxes[op.box_number()].apply(op);
            });
        Some(
            boxes
                .iter()
                .enumerate()
                .map(|(i, lens_box)| (i + 1) * lens_box.focusing_power())
                .sum(),
        )
    }
}

fn xmas_hash(input: &str) -> usize {
    input
        .chars()
        .map(u32::from)
        .fold(0, |acc, x| ((acc + x) * 17) % 256) as usize
}

#[derive(Debug)]
struct LensBox<'a> {
    lenses: Vec<Lens<'a>>,
}

impl<'a> LensBox<'a> {
    fn pos_lens_with_label(&self, label: &str) -> Option<usize> {
        self.lenses.iter().position(|lens| lens.label == label)
    }

    pub fn apply(&mut self, op: Operation<'a>) {
        match (op.clone(), self.pos_lens_with_label(op.label())) {
            (Operation::Remove(_), None) => {}
            (Operation::Remove(_), Some(pos)) => {
                self.lenses.remove(pos);
            }
            (Operation::Set(_, lens), None) => self.lenses.push(lens),
            (Operation::Set(_, lens), Some(pos)) => {
                self.lenses.push(lens);
                self.lenses.swap_remove(pos);
            }
        }
    }

    pub fn focusing_power(&self) -> usize {
        self.lenses
            .iter()
            .enumerate()
            .map(|(i, lens)| (i + 1) * lens.focal_length)
            .sum()
    }
}

#[derive(Debug, Clone)]
struct Lens<'a> {
    label: &'a str,
    focal_length: usize,
}

impl<'a> From<&'a str> for Lens<'a> {
    fn from(s: &'a str) -> Self {
        let (label, f_length) = s.split_once('=').unwrap();
        Self {
            label,
            focal_length: f_length.parse().unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
enum Operation<'a> {
    Remove(&'a str),
    Set(&'a str, Lens<'a>),
}

impl<'a> Operation<'a> {
    pub fn label(&self) -> &str {
        match self {
            Self::Remove(x) => x,
            Self::Set(x, _) => x,
        }
    }

    pub fn box_number(&self) -> usize {
        xmas_hash(self.label())
    }
}

impl<'a> From<&'a str> for Operation<'a> {
    fn from(s: &'a str) -> Self {
        if s.ends_with('-') {
            Self::Remove(&s[0..s.len() - 1])
        } else {
            let label = s.split_once('=').unwrap().0;
            Self::Set(label, s.into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_input;

    #[test]
    fn test_part1_example() {
        let input = read_input(15, true, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(1320))
    }
    #[test]
    fn test_part1_challenge() {
        let input = read_input(15, false, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(495972))
    }
    #[test]
    fn test_part2_example() {
        let input = read_input(15, true, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(145))
    }
    #[test]
    fn test_part2_challenge() {
        let input = read_input(15, false, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(245223))
    }
}
