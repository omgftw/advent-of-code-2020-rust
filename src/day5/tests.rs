use crate::day5;
use std::fs;

#[test]
fn test_data() -> eyre::Result<()> {
    let test_data = fs::read_to_string("src/day5/data/test_1.txt").unwrap();
    let result = day5::day5(Some(test_data.clone()))?;
    // Part 1
    assert_eq!(result.0, 820);
    // Part 2
    // No example given
    Ok(())
}

#[test]
fn main() -> eyre::Result<()> {
    let result = day5::day5(None)?;
    // Part 1
    assert_eq!(result.0, 848);
    // Part 2
    assert_eq!(result.1, 682);

    Ok(())
}
