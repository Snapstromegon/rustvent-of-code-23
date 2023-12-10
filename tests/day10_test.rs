use rustvent::day10::Day;
use rustvent::solution::Solution;
use rustvent::utils::read_input;

#[test]
fn part1_example() {
    let input = read_input(10, true, 1).unwrap();
    assert_eq!(Day.part1(&input), Some(4));
}

#[test]
fn part1_challenge() {
    let input = read_input(10, false, 1).unwrap();
    assert_eq!(Day.part1(&input), Some(6733));
}

#[test]
fn part2_example() {
    let input = read_input(10, true, 2).unwrap();
    assert_eq!(Day.part2(&input), Some(8));
}

#[test]
#[ignore]
fn part2_challenge() {
    let input = read_input(10, false, 2).unwrap();
    assert_eq!(Day.part2(&input), Some(54530));
}
