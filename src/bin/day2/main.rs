use std::collections::LinkedList;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::BitXor;

struct PassSpec {
    first: usize,
    second: usize,
    c: char,
    password: String,
}

fn main() {
    println!("Official:");
    count_valid("src/bin/day2/input");

    println!("Big input:");
    count_valid("src/bin/day2/input_big");
}

fn count_valid(path: &str) {
    let lines = File::open(path)
        .map(BufReader::new)
        .map(|br| br.lines().map(|l| l.unwrap()))
        .unwrap()
        .map(|l| to_spec(l))
        .collect::<LinkedList<PassSpec>>();

    let vp1 = lines
        .iter()
        .filter(|spec| check_p1(spec))
        .collect::<Vec<&PassSpec>>();
    println!("Part 1: Found {} valid passwords", vp1.len());

    let vp2 = lines
        .iter()
        .filter(|spec| check_p2(spec))
        .collect::<Vec<&PassSpec>>();
    println!("Part 2: Found {} valid password", vp2.len());
}

fn to_spec(line: String) -> PassSpec {
    let groups = line.split(" ").collect::<Vec<&str>>();

    let minmax = groups[0].split("-").collect::<Vec<&str>>();

    return PassSpec {
        first: minmax[0].parse::<usize>().unwrap(),
        second: minmax[1].parse::<usize>().unwrap(),
        c: groups[1].chars().nth(0).unwrap(),
        password: groups[2].parse().unwrap(),
    };
}

fn check_p1(spec: &PassSpec) -> bool {
    let count = spec.password.chars().filter(|&c| spec.c == c).count();
    return count >= spec.first && count <= spec.second;
}

fn check_p2(spec: &PassSpec) -> bool {
    let first = spec
        .password
        .chars()
        .nth(spec.first - 1)
        .map(|c| c == spec.c)
        .unwrap_or_else(|| false);
    let second = spec
        .password
        .chars()
        .nth(spec.second - 1)
        .map(|c| c == spec.c)
        .unwrap_or_else(|| false);
    return first.bitxor(second);
}
