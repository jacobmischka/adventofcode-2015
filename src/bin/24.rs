use std::{
    collections::HashSet,
    io::{self, BufRead},
};

use itertools::Itertools;

fn main() {
    let package_weights: Vec<u64> = io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .filter_map(|s| s.parse().ok())
        .collect();

    let ideal_first_group = get_ideal_first_group(&package_weights, 3);
    let part_1 = ideal_first_group.iter().copied().product::<u64>();

    println!("Part 1: {}", part_1);

    let ideal_first_group = get_ideal_first_group(&package_weights, 4);
    let part_2 = ideal_first_group.iter().copied().product::<u64>();

    println!("Part 2: {}", part_2);
}

fn get_ideal_first_group(package_weights: &Vec<u64>, num_groups: u64) -> Vec<u64> {
    let mut possible_groups: HashSet<Vec<u64>> = HashSet::new();

    let target_weight = package_weights.iter().copied().sum::<u64>() / num_groups;

    for i in 1..package_weights.len() {
        for combo in package_weights.iter().copied().combinations(i) {
            if combo.iter().copied().sum::<u64>() == target_weight {
                possible_groups.insert(combo);
            }
        }

        if !possible_groups.is_empty() {
            break;
        }
    }

    possible_groups.into_iter().fold(Vec::new(), |acc, group| {
        if acc.is_empty() || group.len() < acc.len() {
            group
        } else if group.len() == acc.len()
            && group.iter().copied().product::<u64>() < acc.iter().copied().product::<u64>()
        {
            group
        } else {
            acc
        }
    })
}
