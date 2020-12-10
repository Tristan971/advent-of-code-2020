use std::collections::{HashMap, HashSet};

use advent_of_code_2020::common::inputs;

fn main() {
    let mut adapters = inputs::fread_lines("src/bin/day10/input")
        .iter()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    adapters.sort_unstable();

    part1(&adapters);
    part2(&adapters);
}

fn part1(adapters: &Vec<i32>) {
    let mut diff1 = 0;
    let mut diff3 = 0;

    for i in 0..adapters.len() {
        let cur = adapters.get(i).unwrap();
        let pre = match i {
            0 => &0,
            n => adapters.get(n - 1).unwrap(),
        };

        match cur - pre {
            1 => diff1 += 1,
            3 => diff3 += 1,
            _ => {}
        }
    }

    diff3 += 1;

    println!("Silver: {} (1 -> {}, 3 -> {})", diff1 * diff3, diff1, diff3);
}

fn part2(adapters: &Vec<i32>) {
    let lookup = adapters.iter().map(|&a| a).collect::<HashSet<i32>>();

    let display = adapters.get(adapters.len() - 1).unwrap() + 3;
    let permutations = count_permutations_to(display, &lookup, &mut HashMap::new());
    println!("Permutations: {}", permutations);
}

fn count_permutations_to(
    adapter: i32,
    adapters: &HashSet<i32>,
    memoize: &mut HashMap<i32, i64>,
) -> i64 {
    let mut predecessors = 0;
    for diff in 1..4 {
        let predecessor = adapter - diff;
        if predecessor == 0 {
            predecessors += 1;
        }

        if adapters.contains(&predecessor) {
            if memoize.contains_key(&predecessor) {
                predecessors += memoize.get(&predecessor).unwrap();
            } else {
                predecessors += count_permutations_to(predecessor, adapters, memoize);
            }
        }
    }

    memoize.insert(adapter, predecessors);
    return predecessors;
}
