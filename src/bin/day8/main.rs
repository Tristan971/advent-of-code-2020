use std::collections::HashSet;

use advent_of_code_2020::common::inputs;

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    println!("- Part 1 -");
    let lines = inputs::fread_lines("src/bin/day8/input");
    let result = run_and_validate(&lines);
    println!("Ran with result {}", result.0);
}

fn part_2() {
    println!("- Part 2 -");
    let lines = inputs::fread_lines("src/bin/day8/input");

    for i in 0..lines.len() {
        let line = lines.get(i).unwrap();
        if line.starts_with("nop") {
            let replacement_line = line.replace("nop", "jmp");
            let mut replacement_input = lines.clone();
            replacement_input[i] = replacement_line;
            let result = run_and_validate(&replacement_input);
            if result.1 {
                println!(
                    "Swapping nop for jmp at index {} ran correctly with outcome {}",
                    i, result.0
                );
            }
        } else if line.starts_with("jmp") {
            let replacement_line = line.replace("jmp", "nop");
            let mut replacement_input = lines.clone();
            replacement_input[i] = replacement_line;
            let result = run_and_validate(&replacement_input);
            if result.1 {
                println!(
                    "Swapping jmp for nop at index {} ran correctly with outcome {}",
                    i, result.0
                );
            }
        }
    }
}

fn run_and_validate(lines: &Vec<String>) -> (i32, bool) {
    let correct_finishing_pc = lines.len();

    let mut var = 0;

    let mut seen: HashSet<usize> = HashSet::new();

    let mut pc: usize = 0;
    while !seen.contains(&pc) && pc < correct_finishing_pc {
        seen.insert(pc);

        let line = lines.get(pc).unwrap().split(" ").collect::<Vec<&str>>();
        let op = line[0];
        let val: i32 = line[1].parse::<i32>().unwrap();

        match op {
            "acc" => {
                var += val;
                pc += 1;
            }
            "nop" => {
                pc += 1;
            }
            "jmp" => {
                if val > 0 {
                    pc += val as usize;
                } else {
                    pc -= (-val) as usize;
                }
            }
            _ => {
                panic!("Unknown op: {}", op);
            }
        }
    }

    let valid = pc == correct_finishing_pc;
    return (var, valid);
}
