use std::fs;
use crate::day1;

#[test]
fn test_day1_test_data() {
    let test_data = fs::read_to_string("src/day1/data/test_1.txt").unwrap();
    // Part 1
    assert_eq!(day1::day1(false, Some(test_data.clone())), 514579);
    // Part 2
    assert_eq!(day1::day1(true, Some(test_data)), 241861950);
}

#[test]
fn test_day1() {
    // Part 1
    assert_eq!(day1::day1(false, None), 32064);
    // Part 2
    assert_eq!(day1::day1(true, None), 193598720);
}
