use crate::day7;
use std::fs;

#[test]
fn test_data() -> eyre::Result<()> {
    let test_data = fs::read_to_string("src/day7/data/test_1.txt").unwrap();
    let result = day7::day7(Some(test_data.clone()))?;
    // Part 1
    assert_eq!(result.0, 4);
    // Part 2
    let test_data = fs::read_to_string("src/day7/data/test_2.txt").unwrap();
    let result = day7::day7(Some(test_data))?;
    assert_eq!(result.1, 126);
    Ok(())
}

#[test]
fn main() -> eyre::Result<()> {
    let result = day7::day7(None)?;
    // Part 1
    assert_eq!(result.0, 252);
    // Part 2
    assert_eq!(result.1, 35487);
    Ok(())
}
