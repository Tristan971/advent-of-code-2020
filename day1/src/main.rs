use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("Day1!");

    println!("Input");
    parts("input", &2020);

    println!("BIG input");
    parts("input_big", &99920044);
}

fn parts(input: &str, sum: &i64) {
    let lines_iter = match File::open(input) {
        Ok(file) => BufReader::new(file).lines(),
        Err(err) => panic!(err),
    };

    let expenses = lines_iter
        .map(|l| l.unwrap().parse::<i64>().unwrap())
        .collect::<HashSet<i64>>();

    // part 1
    let (p1v1, p1v2) = find2_n(sum, &expenses).unwrap();
    println!("P1: {} * {} = {}", p1v1, p1v2, p1v1 * p1v2);

    // part 2
    let (p2v1, p2v2, p3v3) = expenses
        .iter()
        .find_map(|&v1| find2_n(&(sum - v1), &expenses).map(|(v2, v3)| (v1, v2, v3)))
        .unwrap();
    println!(
        "P2: {} * {} * {} = {}",
        p2v1,
        p2v2,
        p3v3,
        p2v1 * p2v2 * p3v3
    );
}

fn find2_n(n: &i64, expenses: &HashSet<i64>) -> Option<(i64, i64)> {
    return expenses
        .iter()
        .find_map(|&v1| expenses.get(&(n - v1)).map(|&v2| (v1, v2)));
}
