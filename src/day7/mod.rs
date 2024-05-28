use std::collections::{HashMap, HashSet};
use std::fs;
use eyre::Result;
use crate::helpers::OptionExt;

#[cfg(test)]
mod tests;

fn get_parent_bags(bags: &HashMap<String, HashMap<String, i32>>, bag: &str) -> (HashSet<String>, i32) {
    let mut total = 0;
    let mut parents = HashSet::new();
    for key in bags.keys() {
        if bags[key].contains_key(bag) {
            total += bags[key][bag];
            parents.insert(key.to_string());
            let (p, c) = get_parent_bags(bags, key);
            parents.extend(p);
            total += c;
        }
    }
    (parents, total)
}

fn get_child_bags(bags: &HashMap<String, HashMap<String, i32>>, bag: &str) -> i32 {
    let mut total = 0;
    if let Some(children) = bags.get(bag) {
        for (child, count) in children {
            total += count + count * get_child_bags(bags, child);
        }
    }
    total
}

pub(crate) fn day7(data: Option<String>) -> Result<(i32, i32)> {
    let data = data.unwrap_or_else(|| fs::read_to_string("src/day7/data/main.txt").unwrap());

    let mut all_bags: HashMap<String, HashMap<String, i32>> = HashMap::new();

    for line in data.lines() {
        let mut split = line.split(" contain ");
        let top_bag = split.next().to_result()?.split(' ');
        let top_bag = top_bag.collect::<Vec<&str>>();
        let top_bag = top_bag[0..top_bag.len() - 1].join(" ");

        let bags = split.next().to_result()?;
        let bags = bags.split(", ");

        let top_bag = all_bags.entry(top_bag.to_string()).or_default();
        for bag in bags {
            let bag = bag.split(' ').collect::<Vec<&str>>();
            let count = bag[0].parse::<i32>().unwrap_or(0);
            let bag_name = bag[1..bag.len() - 1].join(" ");
            top_bag.insert(bag_name, count);
        }
    }

    let find_bag = "shiny gold";
    let (parent_bags, _) = get_parent_bags(&all_bags, find_bag);
    let child_bags = get_child_bags(&all_bags, find_bag);

    Ok((parent_bags.len() as i32, child_bags))
}
