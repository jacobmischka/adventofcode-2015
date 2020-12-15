use itertools::Itertools;

use std::io::{self, BufRead};

const AMOUNT_EGGNOG: u16 = 150;

fn main() {
    let containers: Vec<u16> = io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .filter_map(|s| s.parse().ok())
        .collect();

    let mut num_combos = 0;
    let mut min_num = usize::MAX;
    let mut min_count = 0;
    for size in 2..containers.len() {
        for combo in containers.iter().combinations(size) {
            if combo.into_iter().copied().sum::<u16>() == AMOUNT_EGGNOG {
                num_combos += 1;
                if size < min_num {
                    min_num = size;
                }

                if size == min_num {
                    min_count += 1;
                }
            }
        }
    }

    println!("Part 1: {}", num_combos);
    println!("Part 2: {}", min_count);
}
