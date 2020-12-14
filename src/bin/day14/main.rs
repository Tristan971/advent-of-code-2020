use std::collections::HashMap;
use std::iter::FromIterator;

use advent_of_code_2020::common::inputs;

fn main() {
    let lines = inputs::fread_lines("src/bin/day14/input");

    part_1(&lines);
    part_2(&lines);
}

fn part_1(lines: &Vec<String>) {
    let mut memory: HashMap<u64, u64> = HashMap::new();

    let mut mask: Vec<char> = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"
        .chars()
        .collect::<Vec<char>>();

    for line in lines {
        let parts = line.split(" = ").collect::<Vec<&str>>();
        let action = parts[0];
        let value = parts[1];

        match action {
            "mask" => {
                mask = value.chars().collect::<Vec<char>>();
            }
            _ => {
                let mem_address = parse_address_from_assignation_str(action);

                let original_value = format!("{:b}", value.parse::<u64>().unwrap());

                let mut value_bytes = original_value.chars().collect::<Vec<char>>();
                while value_bytes.len() < 36 {
                    value_bytes.insert(0, '0'); // left-pad value bytes
                }

                for i in 0..36 {
                    match mask[i] {
                        'X' => {}
                        '0' | '1' => {
                            value_bytes[i] = mask[i];
                        }
                        mb => panic!("Unexpected mask byte: {}", mb),
                    }
                }

                let result_str = String::from_iter(value_bytes);
                let inserted = u64::from_str_radix(result_str.as_str(), 2).unwrap();

                memory.insert(mem_address, inserted);
            }
        }
    }

    let sum_of_values: u64 = memory.values().sum();
    println!("Silver: {}", sum_of_values);
}

fn part_2(lines: &Vec<String>) {
    let mut memory: HashMap<u64, u64> = HashMap::new();

    let mut mask: Vec<char> = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"
        .chars()
        .collect::<Vec<char>>();

    for line in lines {
        let parts = line.split(" = ").collect::<Vec<&str>>();
        let action = parts[0];
        let value = parts[1];

        match action {
            "mask" => {
                mask = value.chars().collect::<Vec<char>>();
            }
            _ => {
                let mem_address = parse_address_from_assignation_str(action);

                let value = value.parse::<u64>().unwrap();

                let mut mem_bytes = format!("{:b}", mem_address).chars().collect::<Vec<char>>();
                while mem_bytes.len() < 36 {
                    mem_bytes.insert(0, '0');
                }

                let mut mem_bytes_masked: Vec<char> = Vec::new();
                for i in 0..36 {
                    match mask[i] {
                        '0' => mem_bytes_masked.push(mem_bytes[i]),
                        x => mem_bytes_masked.push(x),
                    }
                }

                //println!(
                //    "Mem bytes fmt: {}",
                //    String::from_iter(mem_bytes_masked.iter())
                //);

                let combinations = combinatorics_mem_bytes(&mem_bytes_masked, 0);
                for combination in combinations {
                    let combination_str = String::from_iter(combination.iter());
                    let memory_address = u64::from_str_radix(combination_str.as_str(), 2).unwrap();
                    //println!("- {} (decimal {})", combination_str, memory_address);

                    memory.insert(memory_address, value);
                }
            }
        }
    }

    let sum_of_values: u64 = memory.values().sum();
    println!("Gold: {}", sum_of_values);
}

fn parse_address_from_assignation_str(action: &str) -> u64 {
    action
        .chars()
        .skip(4) // mem[
        .take_while(|&c| c != ']') // ]
        .collect::<String>()
        .parse::<u64>()
        .unwrap()
}

fn combinatorics_mem_bytes(mem_bytes: &Vec<char>, i: usize) -> Vec<Vec<char>> {
    if i == 36 {
        let mut res = Vec::new();
        res.push(mem_bytes.to_owned());
        return res;
    }

    if mem_bytes[i] == 'X' {
        let mut combinations: Vec<Vec<char>> = Vec::new();

        let mut opt_0 = mem_bytes.clone();
        opt_0[i] = '0';
        let opts_0 = combinatorics_mem_bytes(&opt_0, i + 1);
        opts_0
            .iter()
            .for_each(|opt| combinations.push(opt.to_owned()));

        let mut opt_1 = mem_bytes.clone();
        opt_1[i] = '1';
        let opts_1 = combinatorics_mem_bytes(&opt_1, i + 1);
        opts_1
            .iter()
            .for_each(|opt| combinations.push(opt.to_owned()));

        return combinations;
    } else {
        return combinatorics_mem_bytes(mem_bytes, i + 1);
    }
}
