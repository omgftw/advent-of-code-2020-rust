use std::collections::HashMap;
use std::fs;
use eyre::Result;

#[cfg(test)]
mod tests;

pub(crate) fn day6(data: Option<String>) -> Result<(i32, i32)> {
    let data = data.unwrap_or_else(|| fs::read_to_string("src/day6/data/main.txt").unwrap());
    let groups: Vec<&str> = data.split("\n\n").collect();
    let mut any_yes_total = 0;
    let mut all_yes_total = 0;

    for group in groups.iter() {
        let mut yes_answers = HashMap::new();
        for person in group.lines() {
            for letter in person.chars() {
                yes_answers.insert(letter, yes_answers.get(&letter).unwrap_or(&0) + 1);
            }
        }
        any_yes_total += yes_answers.len();
        all_yes_total += yes_answers.iter().filter(|(_, &v)| v == group.lines().count()).count();
    }

    Ok((any_yes_total as i32, all_yes_total as i32))
}
