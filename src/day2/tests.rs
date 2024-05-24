use crate::day2;
use std::fs;

#[test]
fn test_data() {
    let test_data = fs::read_to_string("src/day2/data/test_1.txt").unwrap();
    // Part 1
    assert_eq!(day2::day2_part1(Some(test_data.clone())), 2);
    // Part 2
    assert_eq!(day2::day2_part2(Some(test_data)), 1);
}

#[test]
fn main() {
    // Part 1
    assert_eq!(day2::day2_part1(None), 458);
    // Part 2
    assert_eq!(day2::day2_part2(None), 342);
}
