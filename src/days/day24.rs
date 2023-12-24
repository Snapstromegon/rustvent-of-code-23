use nalgebra::{Vector2, Vector3, Matrix2};
use std::str::FromStr;

use crate::solution::Solution;

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<usize> {
        let hail = input
            .lines()
            .map(|l| l.trim().parse::<Hailstone>().unwrap())
            .collect::<Vec<_>>();
        let mut intersections = vec![];
        for i in 0..hail.len() {
            for j in i + 1..hail.len() {
                if let Some(intersection) = hail[i].intersects_2s(&hail[j]) {
                    intersections.push((intersection, i, j));
                }
            }
        }
        let intersections = intersections
            .iter()
            .filter(|(intersection, _, _)| in_bounds_2d(*intersection, (200000000000000.0, 400000000000000.0)))
            .collect::<Vec<_>>();
        Some(intersections.len())
    }

    fn part2(&self, _input: &str) -> Option<usize> {
        None
    }
}

fn in_bounds_2d(intersection: Vector2<f64>, range: (f64, f64)) -> bool {
    intersection.x >= range.0
        && intersection.x <= range.1
        && intersection.y >= range.0
        && intersection.y <= range.1
}

fn solve_system(a: Vector2<f64>, b: Vector2<f64>, c: Vector2<f64>) -> Option<Vector2<f64>> {
    // Hier wird `nalgebra` verwendet, um das Gleichungssystem zu lösen.
    // Falls es keine Lösung gibt, wird `None` zurückgegeben.
    let x = Matrix2::new(a.x, b.x, a.y, b.y).qr().solve(&c)?;

    Some(x)
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Hailstone {
    position: Vector3<f64>,
    velocity: Vector3<f64>,
}

impl Hailstone {
    fn position_2d(&self) -> Vector2<f64> {
        Vector2::new(self.position.x, self.position.y)
    }

    fn velocity_2d(&self) -> Vector2<f64> {
        Vector2::new(self.velocity.x, self.velocity.y)
    }

    pub fn intersects_2s(&self, other: &Self) -> Option<Vector2<f64>> {
        let a = self.velocity_2d();
        let b = -other.velocity_2d();
        let c: Vector2<f64> = other.position_2d() - self.position_2d();

        let t_values = solve_system(a, b, c)?;

        let intersection_point = self.position_2d() + t_values.x * self.velocity_2d();

        // Test if intersection was in the past
        if t_values.x < 0.0 || t_values.y < 0.0 {
            return None;
        }

        Some(intersection_point)
    }
}

impl FromStr for Hailstone {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("@");
        let position_numbers = split
            .next()
            .unwrap()
            .split(", ")
            .map(|s| s.trim().parse().unwrap());
        let position = Vector3::from_iterator(position_numbers);
        let velocity_numbers = split
            .next()
            .unwrap()
            .split(", ")
            .map(|s| s.trim().parse().unwrap());
        let velocity = Vector3::from_iterator(velocity_numbers);
        Ok(Hailstone { position, velocity })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solution::Solution;
    use crate::utils::read_input;

    #[test]
    fn test_part1_example() {
        let input = read_input(0, true, 1).unwrap();
        assert_eq!(Day.part1(&input), None);
    }
    #[test]
    fn test_part1_challenge() {
        let input = read_input(0, false, 1).unwrap();
        assert_eq!(Day.part1(&input), None);
    }

    #[test]
    fn test_part2_example() {
        let input = read_input(0, true, 2).unwrap();
        assert_eq!(Day.part2(&input), None);
    }
    #[test]
    fn test_part2_challenge() {
        let input = read_input(0, false, 2).unwrap();
        assert_eq!(Day.part2(&input), None);
    }
}
