use eyre::Result;
use std::fs;

#[cfg(test)]
mod tests;

struct Instruction {
    operation: String,
    argument: i32,
    has_run: bool,
}

pub(crate) fn day8(data: Option<String>) -> Result<(i32, i32)> {
    let data = data.unwrap_or_else(|| fs::read_to_string("src/day8/data/main.txt").unwrap());

    let mut instructions = Vec::new();
    for line in data.lines() {
        let instruction = line.split_whitespace().collect::<Vec<&str>>();
        let instruction = Instruction {
            operation: instruction[0].to_string(),
            argument: instruction[1].parse::<i32>().unwrap(),
            has_run: false,
        };
        instructions.push(instruction);
    }

    let mut accumulator = 0;

    let mut index = 0;
    loop {
        let instruction = &mut instructions[index];
        if instruction.has_run {
            break;
        }
        instruction.has_run = true;
        match instruction.operation.as_str() {
            "acc" => {
                accumulator += instruction.argument;
                index += 1;
            }
            "jmp" => {
                index = (index as i32 + instruction.argument) as usize;
            }
            "nop" => {
                index += 1;
            }
            _ => {
                panic!("Unknown operation: {}", instruction.operation);
            }
        }
    }

    Ok((accumulator, 0))
}
