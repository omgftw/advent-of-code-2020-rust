use std::fs;
use crate::day2;

#[test]
fn test_day1_test_data() {
    let test_data = fs::read_to_string("src/day2/data/test_1.txt").unwrap();
    // Part 1
    assert_eq!(day2::day2_part1(Some(test_data.clone())), 2);
    // Part 2
    // assert_eq!(day1::day1(Some(test_data)), 241861950);
}

#[test]
fn test_day1() {
    // Part 1
    // assert_eq!(day1::day1(false, None), 32064);
    // Part 2
    // assert_eq!(day1::day1(true, None), 193598720);
}
