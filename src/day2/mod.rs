use regex::Regex;
use std::fs;
use lazy_static::lazy_static;

#[cfg(test)]
mod tests;

lazy_static! {
    static ref DATA_REGEX: Regex = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
}

pub fn parse_line(line: &str, regex: &Regex) -> (i32, i32, char, String) {
    let captures = regex.captures(line).unwrap();
    let min = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
    let max = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
    let letter = captures.get(3).unwrap().as_str().parse::<char>().unwrap();
    let password = captures.get(4).unwrap().as_str().to_string();

    (min, max, letter, password)
}

pub(crate) fn day2_part1(data: Option<String>) -> i32 {
    let data = data.unwrap_or_else(|| fs::read_to_string("src/day2/data/main.txt").unwrap());

    let mut valid_passwords = 0;
    for line in data.lines() {
        let (min, max, letter, password) = parse_line(line, &DATA_REGEX);

        let mut letter_count = 0;
        for c in password.chars() {
            if c == letter {
                letter_count += 1;
            }
        }

        if letter_count >= min && letter_count <= max {
            valid_passwords += 1;
        }
    }

    valid_passwords
}

pub(crate) fn day2_part2(data: Option<String>) -> i32 {
    let data = data.unwrap_or_else(|| fs::read_to_string("src/day2/data/main.txt").unwrap());

    let mut valid_passwords = 0;
    for line in data.lines() {
        let (min, max, letter, password) = parse_line(line, &DATA_REGEX);

        let first_char = password.chars().nth((min - 1) as usize).unwrap();
        let second_char = password.chars().nth((max - 1) as usize).unwrap();

        if (first_char == letter) ^ (second_char == letter) {
            valid_passwords += 1;
        }
    }

    valid_passwords
}
