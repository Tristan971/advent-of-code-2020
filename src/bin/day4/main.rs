#[macro_use]
extern crate lazy_static;

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, Read};

use regex::Regex;

lazy_static! {
    static ref R_BYR: Regex = Regex::new(r"^(19[2-9][0-9])|(200[0-2])$").unwrap();
    static ref R_IYR: Regex = Regex::new(r"^20(1[0-9]|20)$").unwrap();
    static ref R_EYR: Regex = Regex::new(r"^20(2[0-9]|30)$").unwrap();
    static ref R_HGT: Regex =
        Regex::new(r"^(1(([5-8][0-9])|(9[0-3]))cm)|(((59)|(6[0-9])|(7[0-6]))in)$").unwrap();
    static ref R_HCL: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    static ref R_ECL: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    static ref R_PID: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
}

fn main() {
    println!("Input OFFICIAL:");
    run_input("src/bin/day4/input");

    println!("Input BIG:");
    run_input("src/bin/day4/input_big");
}

fn run_input(input: &str) {
    let mut content = String::new();
    File::open(input)
        .map(BufReader::new)
        .unwrap()
        .read_to_string(&mut content)
        .unwrap();

    let part1 = content.split("\n\n").filter(|&doc| part_1(doc)).count();
    println!("Part 1: {}", part1);

    let part2 = content.split("\n\n").filter(|&doc| part_2(doc)).count();
    println!("Part 2: {}", part2);
}

fn part_2(document: &str) -> bool {
    return document
        .split_whitespace()
        .filter(|idfield| {
            let parts = idfield.split(":").collect::<Vec<&str>>();
            if parts.len() != 2 {
                return false;
            }

            let val = parts[1];
            return match parts[0] {
                "byr" => R_BYR.is_match(val),
                "iyr" => R_IYR.is_match(val),
                "eyr" => R_EYR.is_match(val),
                "hgt" => R_HGT.is_match(val),
                "hcl" => R_HCL.is_match(val),
                "ecl" => R_ECL.is_match(val),
                "pid" => R_PID.is_match(val),
                _ => false,
            };
        })
        .count()
        >= 7;
}

fn part_1(document: &str) -> bool {
    let fields = document
        .split_whitespace()
        .filter_map(|field| field.split(":").nth(0))
        .collect::<HashSet<&str>>();

    return fields.contains("byr")
        && fields.contains("iyr")
        && fields.contains("eyr")
        && fields.contains("hgt")
        && fields.contains("hcl")
        && fields.contains("ecl")
        && fields.contains("pid");
}
