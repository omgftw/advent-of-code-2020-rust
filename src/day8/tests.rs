use crate::day8;
use std::fs;

#[test]
fn test_data() -> eyre::Result<()> {
    let test_data = fs::read_to_string("src/day8/data/test_1.txt").unwrap();
    let result = day8::day8(Some(test_data.clone()))?;
    // Part 1
    assert_eq!(result.0, 5);
    // Part 2
    // let test_data = fs::read_to_string("src/day8/data/test_2.txt").unwrap();
    // let result = day8::day8(Some(test_data))?;
    // assert_eq!(result.1, ?);
    Ok(())
}

#[test]
fn main() -> eyre::Result<()> {
    let result = day8::day8(None)?;
    // Part 1
    assert_eq!(result.0, 1137);
    // Part 2
    // assert_eq!(result.1, ?);
    Ok(())
}
