use crate::helpers::OptionExt;
use eyre::Result;
use std::collections::{HashMap, HashSet};
use std::fs;

#[cfg(test)]
mod tests;

fn get_top_level_parent_bags(
    bags: &HashMap<String, HashSet<String>>,
    bag: &str,
    top_parent_bags: &mut HashSet<String>,
) -> i32 {
    let mut total = 0;
    if let Some(parents) = bags.get(bag) {
        for parent in parents {
            if top_parent_bags.insert(parent.to_string()) {
                total += 1 + get_top_level_parent_bags(bags, parent, top_parent_bags);
            }
        }
    }
    total
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
            if count == 0 {
                continue;
            }
            let bag_name = bag[1..bag.len() - 1].join(" ");
            top_bag.insert(bag_name, count);
        }
    }

    let find_bag = "shiny gold";
    let reverse_bag_map = all_bags
        .iter()
        .fold(HashMap::new(), |mut acc, (key, value)| {
            for (k, _) in value.iter() {
                let entry: &mut HashSet<String> = acc.entry(k.to_string()).or_default();
                entry.insert(key.to_string());
            }
            acc
        });
    let mut top_parent_bags = HashSet::new();
    let parent_bags = get_top_level_parent_bags(&reverse_bag_map, find_bag, &mut top_parent_bags);
    let child_bags = get_child_bags(&all_bags, find_bag);

    Ok((parent_bags, child_bags))
}
