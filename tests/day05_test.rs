use rustvent::day05::Day;
use rustvent::solution::Solution;
use rustvent::utils::read_input;

#[test]
fn part1_example() {
    let input = read_input(5, true, 1);
    assert_eq!(Day.part1(&input), Some(35));
}
#[test]
fn part1_challenge() {
    let input = read_input(5, false, 1);
    assert_eq!(Day.part1(&input), Some(51752125));
}

#[test]
#[ignore = "not yet implemented"]
fn part2_example() {
    let input = read_input(5, true, 2);
    assert_eq!(Day.part2(&input), Some(46));
}
#[test]
#[ignore = "not yet implemented"]
fn part2_challenge() {
    let input = read_input(5, false, 2);
    assert_eq!(Day.part2(&input), Some(12634632));
}
