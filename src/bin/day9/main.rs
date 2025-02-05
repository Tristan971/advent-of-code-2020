use std::collections::HashMap;

use advent_of_code_2020::common::inputs;

fn main() {
    let numbers = inputs::fread_lines("src/bin/day9/input")
        .iter()
        .map(|l| l.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let invalid = part_1(&numbers, 25);
    part_2(&numbers, invalid.unwrap());
}

fn part_1(numbers: &Vec<i64>, window_size: usize) -> Option<i64> {
    println!("- Part 1 -");

    for i in 0..numbers.len() - window_size {
        let window: HashMap<&i64, i32> =
            numbers
                .iter()
                .skip(i)
                .take(window_size)
                .fold(HashMap::new(), |mut sofar, num| {
                    sofar.insert(num, *sofar.get(num).unwrap_or(&0) + 1);
                    return sofar;
                });

        let sum_target = numbers.get(window_size + i).unwrap();

        let couple_exists = window.iter().find_map(|(&pair1, &appearances)| {
            if pair1 > sum_target {
                return None;
            }

            let pair2 = &(sum_target - pair1);
            if pair2 == pair1 && appearances < 2 {
                return None;
            }

            return match window.get_key_value(pair2) {
                Some((&p2, _)) => Some((pair1, p2)),
                None => None,
            };
        });
        if couple_exists.is_none() {
            println!("No complement for {}", sum_target);
            return Some(*sum_target);
        }
    }

    return None;
}

fn part_2(numbers: &Vec<i64>, target: i64) {
    println!("- Part 2 - [searching for: {}]", target);

    let mut result: Option<Vec<&i64>> = None;
    for offset in 0..numbers.len() {
        let mut sum: i64 = 0;
        let group = numbers
            .iter()
            .skip(offset)
            .take_while(|&v| {
                return if sum == target {
                    // already there
                    false
                } else if sum < target {
                    // need more
                    sum += v;
                    true
                } else {
                    // already above
                    false
                };
            })
            .collect::<Vec<&i64>>();

        if sum <= target {
            println!(
                "Range [{}..{}] sums to {}",
                offset,
                offset + group.len(),
                target
            );
            result = Some(group);
            break;
        }
    }

    let mut group = result.unwrap();
    group.sort_unstable();

    let min = *group.get(0).unwrap();
    let max = *group.get(group.len() - 1).unwrap();
    println!("{} + {} = {}", min, max, min + max);
}
