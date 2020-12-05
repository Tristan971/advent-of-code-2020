use std::collections::HashSet;

use advent_of_code_2020::common::inputs;

fn main() {
    println!("Day1!");

    println!("Input");
    parts("src/bin/day1/input", &2020);

    println!("BIG input");
    parts("src/bin/day1/input_big", &99920044);
}

fn parts(input: &str, sum: &i64) {
    let expenses = inputs::fread_lines(input)
        .iter()
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<HashSet<i64>>();

    // part 1
    let (p1v1, p1v2) = find2_n(sum, sum, &expenses).unwrap();
    println!("P1: {} * {} = {}", p1v1, p1v2, p1v1 * p1v2);

    // part 2
    let (p2v1, p2v2, p2v3) = expenses
        .iter()
        .find_map(|v1| find2_n(&(sum - v1), v1, &expenses).map(|(v2, v3)| (*v1, v2, v3)))
        .unwrap();
    let result = p2v1 as i128 * p2v2 as i128 * p2v3 as i128;
    println!("P2: {} * {} * {} = {}", p2v1, p2v2, p2v3, result);
}

fn find2_n(n: &i64, exclude: &i64, expenses: &HashSet<i64>) -> Option<(i64, i64)> {
    return expenses
        .iter()
        .filter(|&v| v < n && v != exclude)
        .find_map(|v1| {
            expenses
                .get(&(n - v1))
                .filter(|&v2| v1 != v2 && v2 != exclude)
                .map(|v2| (*v1, *v2))
        });
}
