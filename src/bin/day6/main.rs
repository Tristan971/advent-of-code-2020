use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, Read};

use regex::internal::Char;

use advent_of_code_2020::common::inputs;

fn main() {
    run_input("src/bin/day6/input");
}

fn run_input(input: &str) {
    let mut content = String::new();
    File::open(input)
        .map(BufReader::new)
        .unwrap()
        .read_to_string(&mut content)
        .unwrap();
    content += "\n";

    let groups = content
        .split("\n\n")
        .map(|gin| read_group(gin))
        .collect::<Vec<HashMap<char, i32>>>();

    let part1: usize = groups.iter().map(|group| group.keys().len() - 1).sum();
    println!("Part 1: {}", part1);

    let part2: usize = groups
        .iter()
        .map(|group| {
            let group_size = group.get(&'\n').unwrap() + 1;
            return group
                .keys()
                .filter(|&k| !k.eq(&'\n'))
                .filter(|&k| group.get(k).unwrap() == &group_size)
                .count();
        })
        .sum();
    println!("Part 2: {}", part2);
}

fn read_group(group_in: &str) -> HashMap<char, i32> {
    let mut appearances: HashMap<char, i32> =
        group_in.chars().fold(HashMap::new(), |mut counts, c| {
            if !counts.contains_key(&c) {
                counts.insert(c, 1);
            } else {
                counts.insert(c, counts.get(&c).unwrap() + 1);
            }
            return counts;
        });
    if !appearances.contains_key(&'\n') {
        appearances.insert('\n', 0);
    }

    println!("Gin: {}", group_in);
    println!("Group: {:#?}", appearances);

    return appearances;
}
