use crate::day6;
use std::fs;

#[test]
fn test_data() -> eyre::Result<()> {
    let test_data = fs::read_to_string("src/day6/data/test_1.txt").unwrap();
    let result = day6::day6(Some(test_data.clone()))?;
    // Part 1
    assert_eq!(result.0, 11);
    // Part 2
    assert_eq!(result.1, 6);
    Ok(())
}

#[test]
fn main() -> eyre::Result<()> {
    let result = day6::day6(None)?;
    // Part 1
    assert_eq!(result.0, 6782);
    // Part 2
    assert_eq!(result.1, 3596);
    Ok(())
}
