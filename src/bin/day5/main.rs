use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut sids = File::open("src/bin/day5/input")
        .map(BufReader::new)
        .map(|br| br.lines().map(|l| l.unwrap()))
        .unwrap()
        .map(|line| to_sid(line))
        .collect::<Vec<u32>>();
    sids.sort_unstable();

    let max_sid = sids[sids.len() - 1];
    println!("Part 1: {}", max_sid);

    for i in 1..(sids.len()) {
        let cur = sids[i];
        let prev = sids[i - 1];

        let expected_cur = prev + 1;
        if cur != expected_cur {
            println!("Part 2: {}", expected_cur);
            return;
        }
    }
}

fn to_sid(boarding_pass: String) -> u32 {
    let bytes = boarding_pass
        .replace("B", "1")
        .replace("R", "1")
        .replace("F", "0")
        .replace("L", "0");

    return u32::from_str_radix(bytes.as_str(), 2).unwrap();
}
