use crate::day4;
use std::fs;

#[test]
fn test_data() -> eyre::Result<()> {
    let test_data = fs::read_to_string("src/day4/data/test_1.txt").unwrap();
    // Part 1
    assert_eq!(day4::day4(false, Some(test_data.clone()))?, 2);
    // Part 2 - invalid
    let test_data = fs::read_to_string("src/day4/data/test_2.txt").unwrap();
    assert_eq!(day4::day4(true, Some(test_data))?, 0);
    // Part 2 - valid
    let test_data = fs::read_to_string("src/day4/data/test_3.txt").unwrap();
    assert_eq!(day4::day4(true, Some(test_data))?, 4);
    Ok(())
}

#[test]
fn main() -> eyre::Result<()> {
    // Part 1
    assert_eq!(day4::day4(false, None)?, 226);
    // Part 2
    assert_eq!(day4::day4(true, None)?, 160);
    Ok(())
}
