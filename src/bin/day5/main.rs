use std::collections::HashSet;
use std::fmt::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

const ROWS: i32 = 128;
const COLS: i32 = 7;

struct Seat {
    row: i32,
    col: i32,
    sid: i32,
}

fn main() {
    let test_bp1 = read_seat(String::from("BFFFBBFRRR"));
    println!("BP1: {} x {}", test_bp1.row, test_bp1.col);

    let test_bp2 = read_seat(String::from("FFFBBBFRRR"));
    println!("BP2: {} x {}", test_bp2.row, test_bp2.col);

    let test_bp3 = read_seat(String::from("BBFFBBFRLL"));
    println!("BP3: {} x {}", test_bp3.row, test_bp3.col);

    let seats = File::open("src/bin/day5/input")
        .map(BufReader::new)
        .map(|br| br.lines().map(|l| l.unwrap()))
        .unwrap()
        .map(|line| read_seat(line))
        .collect::<Vec<Seat>>();

    let max_sid = seats.iter().map(|seat| seat.sid).max();
    println!("Part 1: {}", max_sid.unwrap());

    let mut contiguous_sids = seats
        .iter()
        .filter_map(|seat| match seat.row {
            0 | ROWS => None,
            _ => Some(seat.sid),
        })
        .collect::<Vec<i32>>();
    contiguous_sids.sort_unstable();

    for i in 1..(contiguous_sids.len()) {
        let cur = contiguous_sids[i];
        let prev = contiguous_sids[i - 1];

        let expected_cur = prev + 1;
        if cur != expected_cur {
            println!("Part 2: {}", expected_cur);
            return;
        }
    }
}

fn read_seat(boarding_pass: String) -> Seat {
    let mut rowmin = 0;
    let mut rowmax = ROWS;
    let mut colmin = 0;
    let mut colmax = COLS;

    for char in boarding_pass.chars() {
        let delta_row = (rowmax - rowmin) as f32 / 2.0; // 63.5
        let delta_col = (colmax - colmin) as f32 / 2.0; // 3.5

        match char {
            'B' => {
                rowmin += delta_row.ceil() as i32; // [0 + 64, 127]
            }
            'F' => {
                rowmax -= delta_row.ceil() as i32; // [0, 127 - 64]
            }
            'R' => {
                colmin += delta_col.ceil() as i32; // [0 + 4, 7]
            }
            'L' => {
                colmax -= delta_col.ceil() as i32; // [0, 7 - 4]
            }
            _ => panic!("Invalid char: {}", char),
        }
    }

    return Seat {
        row: rowmin,
        col: colmin,
        sid: rowmin * 8 + colmin,
    };
}
