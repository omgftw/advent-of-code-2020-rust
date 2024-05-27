use std::collections::HashMap;
use std::fs;
use eyre::{ensure, eyre, Result};
use crate::helpers::print_debug;

#[cfg(test)]
mod tests;


pub(crate) fn day4(part2: bool, data: Option<String>) -> Result<i32> {
    let data = data.unwrap_or_else(|| fs::read_to_string("src/day4/data/main.txt").unwrap());
    let passports = data.split("\n\n").map(|x| x.split([' ', '\n']).collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();

    let passports = passports.iter().map(|x| {
        let mut map = HashMap::new();
        for y in x {
            let mut split = y.split(':');
            map.insert(split.next().unwrap(), split.next().unwrap());
        }
        map
    }).collect::<Vec<HashMap<&str, &str>>>();

    let expected_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    type Validation = fn(&str) -> Result<()>;
    let mut validations: HashMap<&str, Validation> = HashMap::new();
    validations.insert("byr", |x: &str| -> Result<()>{
        ensure!(x.len() == 4, "byr must be 4 digits");
        let x = x.parse::<i32>()?;
        ensure!(x >= 1920, "byr too low");
        ensure!(x <= 2002, "byr too high");
        Ok(())
    });
    validations.insert("iyr", |x: &str| -> Result<()>{
        ensure!(x.len() == 4, "iyr must be 4 digits");
        let x = x.parse::<i32>()?;
        ensure!(x >= 2010, "iyr too low");
        ensure!(x <= 2020, "iyr too high");
        Ok(())
    });
    validations.insert("eyr", |x: &str| -> Result<()>{
        ensure!(x.len() == 4, "eyr must be 4 digits");
        let x = x.parse::<i32>()?;
        ensure!(x >= 2020, "eyr too low");
        ensure!(x <= 2030, "eyr too high");
        Ok(())
    });
    validations.insert("hgt", |x: &str| -> Result<()>{
        let unit = &x[x.len()-2..];
        let x = &x[..x.len()-2];
        let x = x.parse::<i32>()?;
        match unit {
            "cm" => {
                ensure!(x >= 150, "hgt too low");
                ensure!(x <= 193, "hgt too high");
            },
            "in" => {
                ensure!(x >= 59, "hgt too low");
                ensure!(x <= 76, "hgt too high");
            },
            _ => return Err(eyre!("Invalid hgt unit"))
        }
        Ok(())
    });
    validations.insert("hcl", |x: &str| -> Result<()>{
        ensure!(x.starts_with('#'), "hcl must start with #");
        ensure!(x.len() == 7, "hcl must be 7 characters");
        for c in x.chars().skip(1) {
            ensure!(c.is_ascii_hexdigit(), "hcl must be hex");
        }
        Ok(())
    });
    validations.insert("ecl", |x: &str| -> Result<()>{
        ensure!(["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&x), "Invalid ecl");
        Ok(())
    });
    validations.insert("pid", |x: &str| -> Result<()>{
        ensure!(x.len() == 9, "pid must be 9 digits");
        for c in x.chars() {
            ensure!(c.is_ascii_digit(), "pid must be digits");
        }
        Ok(())
    });

    let mut invalid_count = 0;
    for passport in passports.iter() {
        let mut cur_success = true;
        print_debug(format!("Passport: {:?}", passport));
        for field in expected_fields {
            if !passport.contains_key(field) {
                invalid_count += 1;
                cur_success = false;
                print_debug(format!("❌ Missing field: {}", field));
                break;
            }
            if part2 {
                let value = passport.get(field).unwrap();
                if let Some(validation) = validations.get(field) {
                    if let Err(e) = validation(value) {
                        invalid_count += 1;
                        cur_success = false;
                        print_debug(format!("❌ Invalid field: {} - {}", field, e));
                        break;
                    }
                }
            }
        }
        if cur_success {
            print_debug("✅ Valid");
        }
        print_debug("");
    }

    Ok(passports.len() as i32 - invalid_count)
}
