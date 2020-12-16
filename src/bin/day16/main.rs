use advent_of_code_2020::common::inputs;
use std::collections::{HashMap, HashSet};

struct FieldRule<'a> {
    name: &'a str,
    r1_min: i32,
    r1_max: i32,
    r2_min: i32,
    r2_max: i32,
}

struct Input<'a> {
    rules: Vec<FieldRule<'a>>,
    self_ticket: Vec<i32>,
    other_tickets: Vec<Vec<i32>>,
}

fn main() {
    let lines = inputs::fread_lines("src/bin/day16/input");

    let input = read_input(&lines);
    print_input(&input);

    part_1(&input);
    part_2(&input);
}

fn part_1(input: &Input) {
    let mut invalids: Vec<i32> = Vec::new();

    for other_ticket in input.other_tickets.iter() {
        for other_ticket_value in other_ticket.iter() {
            let mut valid = false;
            for rule in input.rules.iter() {
                if rule.r1_min <= *other_ticket_value && rule.r1_max >= *other_ticket_value {
                    valid = true;
                    break;
                }

                if rule.r2_min <= *other_ticket_value && rule.r2_max >= *other_ticket_value {
                    valid = true;
                    break;
                }
            }

            if !valid {
                invalids.push(*other_ticket_value);
            }
        }
    }

    println!("Invalid values encountered: {:?}", invalids);
    let error_rate: i32 = invalids.iter().sum();
    println!("Silver: {}", error_rate);
}

fn part_2(input: &Input) {
    let rules_by_name: HashMap<&str, &FieldRule> =
        input.rules.iter().fold(HashMap::new(), |mut map, rule| {
            map.insert(rule.name, rule);
            return map;
        });

    let mut rule_positions: HashMap<usize, HashSet<&str>> = HashMap::new();

    let len_ticket = input.other_tickets[0].len();
    for i in 0..len_ticket {
        let position_rules = rule_positions.entry(i).or_insert_with(|| HashSet::new());
        input.rules.iter().for_each(|rule| {
            position_rules.insert(rule.name);
        });
    }

    for other_ticket in input.other_tickets.iter() {
        if !is_valid_ticket(other_ticket, &input.rules) {
            println!("Skipping invalid ticket: {:?}", other_ticket);
            continue;
        }

        for i in 0..other_ticket.len() {
            let prev_possibilities = rule_positions.get(&i).unwrap();
            let new_possibilities = prev_possibilities
                .iter()
                .map(|rname| rules_by_name.get(rname).unwrap())
                .filter_map(|rule| match validates_rule(other_ticket[i], rule) {
                    true => Some(rule.name),
                    false => None,
                })
                .collect::<HashSet<&str>>();
            rule_positions.insert(i, new_possibilities);
        }
    }

    println!(
        "Pre-flattening, possible rules per position are: {:?}",
        rule_positions
    );

    let mut final_rules: HashMap<&str, usize> = HashMap::new();
    while final_rules.len() < len_ticket {
        let (position, unaccounted_rules) = &rule_positions
            .iter()
            .find(|(_, rules)| {
                rules
                    .iter()
                    .filter(|&&r| !final_rules.contains_key(r))
                    .count()
                    == 1
            })
            .unwrap();

        let certain_pos = *position;
        let certain_rule = unaccounted_rules
            .iter()
            .filter(|&r| !final_rules.contains_key(r))
            .next()
            .unwrap();
        println!("Position {} must be \"{}\"", certain_pos, certain_rule);
        final_rules.insert(certain_rule, *certain_pos);
    }

    println!("Post-flattening rules: {:?}", final_rules);

    let gold: i64 = final_rules
        .iter()
        .filter_map(|(&r, pos)| {
            if r.starts_with("departure") {
                return Some(pos);
            }
            return None;
        })
        .map(|&position| input.self_ticket[position] as i64)
        .product();
    println!("Gold: {}", gold);
}

fn read_input(lines: &Vec<String>) -> Input {
    let mut step = 0;

    let mut rules: Vec<FieldRule> = Vec::new();
    let mut self_ticket: Vec<i32> = Vec::new();
    let mut other_tickets: Vec<Vec<i32>> = Vec::new();

    for line in lines.iter().filter(|s| !s.is_empty()) {
        if line.as_str() == "your ticket:" || line.as_str() == "nearby tickets:" {
            step += 1;
            continue;
        }

        match step {
            0 => {
                let rule_parts = line.split(": ").collect::<Vec<&str>>();
                let name = rule_parts[0];
                let ranges = rule_parts[1].split(" or ").collect::<Vec<&str>>();
                let r1 = ranges[0]
                    .split("-")
                    .map(|i| i.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();
                let r2 = ranges[1]
                    .split("-")
                    .map(|i| i.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();

                rules.push(FieldRule {
                    name,
                    r1_min: r1[0],
                    r1_max: r1[1],
                    r2_min: r2[0],
                    r2_max: r2[1],
                })
            }
            1 => {
                self_ticket = line
                    .split(",")
                    .map(|i| i.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();
            }
            2 => other_tickets.push(
                line.split(",")
                    .map(|i| i.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>(),
            ),
            __ => panic!("Unknown parsing step {}", step),
        }
    }

    return Input {
        rules,
        self_ticket,
        other_tickets,
    };
}

fn print_input(input: &Input) {
    println!("Rules:");
    for rule in input.rules.iter() {
        println!(
            "\t{}: [{}, {}] or [{}, {}]",
            rule.name, rule.r1_min, rule.r1_max, rule.r2_min, rule.r2_max
        );
    }

    println!("Self ticket:");
    println!("\t{:?}", input.self_ticket);

    println!("Nearby tickets:");
    for other_ticket in input.other_tickets.iter() {
        println!("\t{:?}", other_ticket);
    }
}

fn is_valid_ticket(ticket: &Vec<i32>, rules: &Vec<FieldRule>) -> bool {
    for value in ticket.iter() {
        let mut valid = false;
        for rule in rules.iter() {
            valid |= validates_rule(*value, rule);
        }

        if !valid {
            return false;
        }
    }

    return true;
}

fn validates_rule(value: i32, rule: &FieldRule) -> bool {
    if rule.r1_min <= value && rule.r1_max >= value {
        return true;
    }

    if rule.r2_min <= value && rule.r2_max >= value {
        return true;
    }

    return false;
}
