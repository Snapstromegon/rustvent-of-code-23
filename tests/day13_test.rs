use rustvent::day13::Day;
use rustvent::solution::Solution;
use rustvent::utils::read_input;

#[test]
fn part1_example() {
    let input = read_input(13, true, 1).unwrap();
    assert_eq!(Day.part1(&input), Some(405));
}

#[test]
fn part1_challenge() {
    let input = read_input(13, false, 1).unwrap();
    assert_eq!(Day.part1(&input), Some(34100));
}

#[test]
fn part2_example() {
    let input = read_input(13, true, 2).unwrap();
    assert_eq!(Day.part2(&input), Some(400));
}

#[test]
fn part2_challenge() {
    let input = read_input(13, false, 2).unwrap();
    assert_eq!(Day.part2(&input), Some(33106));
}
