use std::collections::{HashMap, HashSet};
use std::ops::{Add, Mul};

use rug::{Float, Integer};

use advent_of_code_2020::common::inputs;

fn main() {
    // official
    run_for_input("src/bin/day10/input");

    // both probably wrong due to bigint stuff
    // big100k
    //run_for_input("src/bin/day10/big_input_100k");
    // big1M
    //run_for_input("src/bin/day10/big_input_1M");
}

fn run_for_input(input: &str) {
    let mut adapters = inputs::fread_lines(input)
        .iter()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    adapters.sort_unstable();

    part1(&adapters);
    part2(&adapters);
}

fn part1(adapters: &Vec<i32>) {
    let mut diff1 = Integer::from(0 as i8);
    let mut diff3 = Integer::from(0 as i8);

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

    let result = diff1.mul(diff3);
    println!("Silver: {}", result);
}

fn part2(adapters: &Vec<i32>) {
    let lookup = adapters.iter().map(|&a| a).collect::<HashSet<i32>>();

    let display = adapters.get(adapters.len() - 1).unwrap() + 3;
    let permutations = count_permutations_to(display, &lookup, &mut HashMap::new());

    println!("Permutations: {}", permutations);
    println!("log10(permutations): {}", log10(permutations))
}

fn count_permutations_to(
    adapter: i32,
    adapters: &HashSet<i32>,
    memoize: &mut HashMap<i32, Integer>,
) -> Integer {
    let mut predecessors = Integer::from(0);
    for diff in 1..4 {
        let predecessor = adapter - diff;
        if predecessor == 0 {
            predecessors += 1 as i8;
        }

        if adapters.contains(&predecessor) {
            if memoize.contains_key(&predecessor) {
                predecessors += memoize.get(&predecessor).unwrap();
            } else {
                predecessors += count_permutations_to(predecessor, adapters, memoize);
            }
        }
    }

    memoize.insert(adapter, predecessors.clone());
    return predecessors;
}

fn log10(i: Integer) -> Float {
    let zero = Float::new(16);
    return zero.add(i).log10();
}
