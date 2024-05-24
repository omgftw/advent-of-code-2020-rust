#[cfg(test)]
mod tests;

pub(crate) fn day1(part2: bool, data: Option<String>) -> i32 {
    let data = data.unwrap_or_else(|| std::fs::read_to_string("src/day1/data/main.txt").unwrap());
    let nums = data.lines().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    for i in 0..nums.len() {
        for j in i..nums.len() {
            if !part2 {
                if nums[i] + nums[j] == 2020 {
                    return nums[i] * nums[j];
                }
            }
            else {
                for k in j..nums.len() {
                    if nums[i] + nums[j] + nums[k] == 2020 {
                        return nums[i] * nums[j] * nums[k];
                    }
                }
            }
        }
    }
    0
}
