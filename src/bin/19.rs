use std::{
    collections::HashSet,
    io::{self, BufRead},
};

fn main() {
    let stdin = io::stdin();
    let mut iter = stdin.lock().lines().filter_map(Result::ok);

    let mut replacements: Vec<(String, String)> = Vec::new();

    while let Some(line) = iter.next() {
        if line.is_empty() {
            break;
        }
        let mut words = line.split_whitespace();
        let from = words.next().unwrap();
        let to = words.skip(1).next().unwrap();
        replacements.push((from.to_string(), to.to_string()));
    }

    let replacements = replacements;
    let medicine_molecule = iter.next().unwrap();

    let mut distinct_molecules = HashSet::new();

    for (from, to) in &replacements {
        for (i, _) in medicine_molecule.match_indices(from) {
            distinct_molecules.insert(replace_at(&medicine_molecule, from, to, i));
        }
    }

    println!("Part 1: {}", distinct_molecules.len());

    let mut molecule = medicine_molecule.clone();
    let mut steps = 0;
    while molecule != "e" {
        steps += 1;
        let (longest_from, longest_to) = replacements
            .iter()
            .filter(|(_from, to)| molecule.contains(to))
            .fold(("", ""), |(longest_from, longest_to), (from, to)| {
                if to.len() > longest_to.len() {
                    (from, to)
                } else {
                    (longest_from, longest_to)
                }
            });

        molecule = molecule.replacen(longest_to, longest_from, 1);
    }

    println!("Part 2: {}", steps);
}

fn replace_at(s: &str, from: &str, to: &str, index: usize) -> String {
    let (start, rest) = s.split_at(index);
    let mut new = String::from(start);
    new.push_str(to);
    new.push_str(&rest[from.len()..]);
    new
}
