use std::collections::HashMap;

use rug::Integer;

use advent_of_code_2020::common::inputs;

fn main() {
    let lines = inputs::fread_lines("src/bin/day13/input");

    println!("Part1:");
    part_1(&lines);
    println!("---\n");

    println!("Part2:");
    part_2(&lines);
}

fn part_1(lines: &Vec<String>) {
    let depart_timestamp = lines.get(0).unwrap().parse::<i32>().unwrap();
    let buses: HashMap<i32, i32> = lines
        .get(1)
        .unwrap()
        .split(",")
        .filter(|&bl| bl != "x")
        .map(|bl| bl.parse::<i32>().unwrap())
        .fold(HashMap::new(), |mut sofar, busid| {
            let fact = depart_timestamp as f32 / busid as f32;
            sofar.insert(busid, fact.ceil() as i32 * busid);
            return sofar;
        });

    let (fastest_busid, pickup_ts) = buses.iter().min_by(|(_, v1), (_, v2)| v1.cmp(v2)).unwrap();
    let wait_time = pickup_ts - depart_timestamp;
    let silver = wait_time * fastest_busid;
    println!(
        "Faster bus is {} passing at {} for a result of {}",
        fastest_busid, pickup_ts, silver
    );
}

fn part_2(lines: &Vec<String>) {
    let input: Vec<&str> = lines.get(1).unwrap().split(",").collect::<Vec<&str>>();

    let mut buses: Vec<(i64, i64)> = Vec::new();
    for i in 0..input.len() {
        let entry = input[i];
        if entry == "x" {
            continue;
        }

        let busid = entry.parse::<i64>().unwrap();
        let offset = i as i64;
        buses.push((busid, offset));
    }

    chinese_remainder(&buses);
}

fn chinese_remainder(constraints: &Vec<(i64, i64)>) {
    println!("Constraints:");

    let mut x: Integer = Integer::from(0);
    constraints
        .iter()
        .map(|&(busid, offset)| (busid, busid - offset))
        .for_each(|(m, r)| {
            println!("\nx === {} [{}]", r, m);

            let prod_other_moduli = constraints
                .iter()
                .filter(|(m2, _)| m != *m2)
                .fold(1, |s, (m2, _)| s * m2);
            println!("Expr: x{} * {} * {}", m, r, prod_other_moduli);

            let ximul: i64 = prod_other_moduli;
            println!("x{} * {} === x{} * {} [{}]", m, ximul, m, ximul % m, m);

            let mut xisol = Integer::from(ximul);
            xisol.invert_mut(&Integer::from(m)).unwrap();
            xisol %= m;

            println!("x{} = {:?}", m, xisol);

            x += prod_other_moduli * r * xisol;
        });

    let prod_all_modulii = constraints
        .iter()
        .fold(1 as i64, |prodall, (m, _)| m * prodall);
    let resx = x.clone() % prod_all_modulii;
    println!("x = {} % {} = {}", x, prod_all_modulii, resx);
}
