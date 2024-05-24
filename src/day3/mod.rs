use std::fs;

#[cfg(test)]
mod tests;

fn traverse_slope(data: &[Vec<char>], movement: (usize, usize)) -> i32 {
    let mut x = 0;
    let mut y = 0;
    let mut trees = 0;
    let width = data[0].len();
    let height = data.len();

    while y < height {
        if data[y][x] == '#' {
            trees += 1;
        }
        x = (x + movement.0) % width;
        y += movement.1;
    }

    trees
}

pub(crate) fn day3(part2: bool, data: Option<String>) -> i64 {
    let data = data.unwrap_or_else(|| fs::read_to_string("src/day3/data/main.txt").unwrap());
    let data = data.lines().collect::<Vec<&str>>();
    let data = data
        .iter()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let movement_sets = match part2 {
        false => vec![(3, 1)],
        true => vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)],
    };

    let mut trees: i64 = 1;
    for movement in movement_sets {
        trees *= traverse_slope(&data, movement) as i64;
    }

    trees
}
