use std::collections::{HashMap, HashSet};

fn main() {
    let data = include_str!("../data/day3.txt")
        .trim()
        .split("\n")
        .map(str::trim);

    let abc = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    let result = data
        .collect::<Vec<&str>>()
        .chunks(3)
        .flat_map(|chunk| {
            chunk
                .iter()
                .flat_map(|line| line.chars().collect::<HashSet<_>>())
                .fold(HashMap::new(), |mut acc: HashMap<char, u32>, c| {
                    *acc.entry(c).or_insert(0) += 1;
                    return acc;
                })
                .into_iter()
                .filter(|(_, count)| *count == 3)
        })
        .map(|(char, _)| abc.find(char).unwrap() + 1)
        .sum::<usize>();

    println!("{:?}", result);
}
