use rustvent::day02::Day;
use rustvent::solution::Solution;
use rustvent::utils::read_input;

#[test]
fn part1_example() {
    let input = read_input(2, true, 1).unwrap();
    assert_eq!(Day.part1(&input), Some(8));
}
#[test]
fn part1_challenge() {
    let input = read_input(2, false, 1).unwrap();
    assert_eq!(Day.part1(&input), Some(2256));
}

#[test]
fn part2_example() {
    let input = read_input(2, true, 2).unwrap();
    assert_eq!(Day.part2(&input), Some(2286));
}
#[test]
fn part2_challenge() {
    let input = read_input(2, false, 2).unwrap();
    assert_eq!(Day.part2(&input), Some(74229));
}
