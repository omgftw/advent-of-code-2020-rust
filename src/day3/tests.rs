use crate::day3;
use std::fs;

#[test]
fn test_data() {
    let test_data = fs::read_to_string("src/day3/data/test_1.txt").unwrap();
    // Part 1
    assert_eq!(day3::day3(false, Some(test_data.clone())), 7);
    // Part 2
    assert_eq!(day3::day3(true, Some(test_data)), 336);
}

#[test]
fn main() {
    // Part 1
    assert_eq!(day3::day3(false, None), 218);
    // Part 2
    assert_eq!(day3::day3(true, None), 3847183340);
}
