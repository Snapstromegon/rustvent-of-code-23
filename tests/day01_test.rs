use rustvent::day01::Day;
use rustvent::solution::Solution;
use rustvent::utils::read_input;

#[test]
fn part1_example() {
    let input = read_input(1, true, 1).unwrap();
    assert_eq!(Day.part1(&input), Some(142));
}

#[test]
fn part1_challenge() {
    let input = read_input(1, false, 1).unwrap();
    assert_eq!(Day.part1(&input), Some(56049));
}

#[test]
fn part2_example() {
    let input = read_input(1, true, 2).unwrap();
    assert_eq!(Day.part2(&input), Some(281));
}

#[test]
fn part2_challenge() {
    let input = read_input(1, false, 2).unwrap();
    assert_eq!(Day.part2(&input), Some(54530));
}
