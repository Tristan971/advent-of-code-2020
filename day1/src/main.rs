use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("Day1!");

    let lines_iter = match File::open("input") {
        Ok(file) => BufReader::new(file).lines(),
        Err(err) => panic!(err),
    };

    let expenses = lines_iter
        .map(|l| l.unwrap())
        .map(|l| l.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    find_couple(&expenses);
    find_trio(&expenses);
}

fn find_couple(expenses: &Vec<i64>) {
    let under_1010 = expenses
        .iter()
        .filter(|e| *e <= &1010)
        .collect::<Vec<&i64>>();

    let over_1010 = expenses
        .iter()
        .filter(|e| *e > &1010)
        .collect::<Vec<&i64>>();

    for se in under_1010 {
        let found = over_1010.iter().find(|&&be| *se + *be == 2020);
        if found.is_some() {
            let be = *found.unwrap();
            let result = se * be;
            println!("SE = {}, BE = {}", se, be);
            println!("Code = {}", result);
        }
    }
}

fn find_trio(expenses: &Vec<i64>) {
    expenses.iter().for_each(|v1| -> () {
        expenses
            .iter()
            .filter(|&v2| v1 + v2 <= 2020)
            .for_each(|v2| -> () {
                expenses
                    .iter()
                    .filter(|&v3| v1 + v2 + v3 <= 2020)
                    .for_each(|v3| -> () {
                        let result = v1 + v2 + v3;
                        let code = v1 * v2 * v3;
                        if result == 2020 {
                            println!("Trio: {} + {} + {} = {}", v1, v2, v3, result);
                            println!("{}", code);
                        }
                    })
            })
    });
}
