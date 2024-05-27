use std::fs;
use std::ops::Range;
use eyre::Result;

#[cfg(test)]
mod tests;

pub(crate) fn day5(data: Option<String>) -> Result<(i32, i32)> {
    let data = data.unwrap_or_else(|| fs::read_to_string("src/day5/data/main.txt").unwrap());

    let mut all_seats = Vec::new();
    let mut max_seat = 0;
    for line in data.lines() {
        let mut rows = Range { start: 0, end: 128 };
        let mut cols = Range { start: 0, end: 8 };
        for letter in line.chars() {
            if letter == 'F' {
                rows.end = (rows.start + rows.end) / 2;
            } else if letter == 'B' {
                rows.start = (rows.start + rows.end) / 2;
            } else if letter == 'L' {
                cols.end = (cols.start + cols.end) / 2;
            } else if letter == 'R' {
                cols.start = (cols.start + cols.end) / 2;
            }
        }
        let seat = rows.start * 8 + cols.start;
        all_seats.push(seat);
        if seat > max_seat {
            max_seat = seat;
        }
    }

    all_seats.sort();

    let mut your_seat = 0;
    for i in all_seats[0]..all_seats[all_seats.len() - 1] {
        if i != all_seats[i - all_seats[0]] {
            your_seat = i;
            break;
        }
    }

    Ok((max_seat as i32, your_seat as i32))
}
