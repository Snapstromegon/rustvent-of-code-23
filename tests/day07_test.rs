use rustvent::day07::Day;
use rustvent::solution::Solution;
use rustvent::utils::read_input;

#[test]
fn part1_example() {
    let input = read_input(7, true, 1);
    assert_eq!(Day.part1(&input), Some(6440));
}
#[test]
fn part1_challenge() {
    let input = read_input(7, false, 1);
    assert_eq!(Day.part1(&input), Some(253603890));
}

#[test]
fn part2_example() {
    let input = read_input(7, true, 2);
    assert_eq!(Day.part2(&input), Some(5905));
}
#[test]
fn part2_challenge() {
    let input = read_input(7, false, 2);
    assert_eq!(Day.part2(&input), Some(253630098));
}
