use std::iter::FromIterator;

use advent_of_code_2020::common::inputs;

struct Direction {
    dx: i32,
    dy: i32,
}

const TL: Direction = Direction { dx: -1, dy: -1 };
const TC: Direction = Direction { dx: 0, dy: -1 };
const TR: Direction = Direction { dx: 1, dy: -1 };
const L: Direction = Direction { dx: -1, dy: 0 };
const R: Direction = Direction { dx: 1, dy: 0 };
const BL: Direction = Direction { dx: -1, dy: 1 };
const BC: Direction = Direction { dx: 0, dy: 1 };
const BR: Direction = Direction { dx: 1, dy: 1 };

const GROUND: char = '.';
const FREE: char = 'L';
const OCCUPIED: char = '#';

fn main() {
    let lines = inputs::fread_lines("src/bin/day11/input");
    let seats = read_initial_seats(&lines);

    // part 1
    println!("- Part 1- ");
    run_with(4, get_occupieds_adj, &seats);

    // part 2
    println!("- Part 2 -");
    run_with(5, get_occupieds_directional, &seats);
}

fn run_with(
    max_occupied_before_vacancy: i32,
    adjacency_counter: fn(usize, usize, &Vec<Vec<char>>) -> i32,
    seats: &Vec<Vec<char>>,
) {
    let mut current_seats = seats.clone();

    let mut changed = true;
    while changed {
        //println!("\nTick");
        //print_seats(&current_seats);

        let changed_and_next: (bool, Vec<Vec<char>>) = tick(
            max_occupied_before_vacancy,
            adjacency_counter,
            &mut current_seats,
        );
        changed = changed_and_next.0;
        current_seats = changed_and_next.1;
        //println!("Changed? {}", changed);
    }

    //println!("\nDone ticking, final arrangement:");
    //print_seats(&current_seats);

    let occupieds = current_seats
        .iter()
        .map(|row| row.iter())
        .flatten()
        .filter(|&&c| c == OCCUPIED)
        .count();
    println!("{} seats occupied after stabilisation", occupieds);
}

fn tick(
    max_before_vacancy: i32,
    counting_mode: fn(usize, usize, &Vec<Vec<char>>) -> i32,
    seats: &mut Vec<Vec<char>>,
) -> (bool, Vec<Vec<char>>) {
    let mut changed: bool = false;
    let mut new_seats: Vec<Vec<char>> = Vec::new();

    for y in 0..seats.len() {
        let row = seats.get(y).unwrap();
        let mut new_row: Vec<char> = Vec::new();

        for x in 0..row.len() {
            let cur_state = seats[y][x];
            let next_state: char = next_state(x, y, max_before_vacancy, counting_mode, seats);

            changed |= cur_state != next_state;
            new_row.push(next_state);
        }

        new_seats.push(new_row);
    }

    return (changed, new_seats);
}

fn next_state(
    x: usize,
    y: usize,
    max_occup_before_vacancy: i32,
    adj_counter: fn(usize, usize, &Vec<Vec<char>>) -> i32,
    seats: &Vec<Vec<char>>,
) -> char {
    return match seats[y][x] {
        GROUND => GROUND,
        FREE => {
            if adj_counter(x, y, seats) == 0 {
                OCCUPIED
            } else {
                FREE
            }
        }
        OCCUPIED => {
            if adj_counter(x, y, seats) >= max_occup_before_vacancy {
                FREE
            } else {
                OCCUPIED
            }
        }
        wot => panic!("Invalid seat state {}:{} is {}", x, y, wot),
    };
}

fn read_initial_seats(lines: &Vec<String>) -> Vec<Vec<char>> {
    return lines
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
}

fn print_seats(seats: &Vec<Vec<char>>) {
    seats
        .iter()
        .for_each(|row| println!("|{}|", String::from_iter(row.iter())));
}

fn get_occupieds_adj(x: usize, y: usize, seats: &Vec<Vec<char>>) -> i32 {
    let mut occupieds = 0;
    if x > 0 && y > 0 {
        occupieds += is_occupied_toi(x - 1, y - 1, seats);
    }

    if x > 0 {
        occupieds += is_occupied_toi(x - 1, y, seats);
        occupieds += is_occupied_toi(x - 1, y + 1, seats);
    }

    if y > 0 {
        occupieds += is_occupied_toi(x, y - 1, seats);
        occupieds += is_occupied_toi(x + 1, y - 1, seats);
    }

    occupieds += is_occupied_toi(x + 1, y, seats);
    occupieds += is_occupied_toi(x, y + 1, seats);
    occupieds += is_occupied_toi(x + 1, y + 1, seats);

    return occupieds;
}

fn get_occupieds_directional(x: usize, y: usize, seats: &Vec<Vec<char>>) -> i32 {
    return [TL, TC, TR, L, R, BL, BC, BR]
        .iter()
        .filter(|&direction| has_occupied_directional(x, y, direction, seats))
        .count() as i32;
}

fn has_occupied_directional(
    x: usize,
    y: usize,
    direction: &Direction,
    seats: &Vec<Vec<char>>,
) -> bool {
    let mut posx: i32 = x as i32 + direction.dx;
    let mut posy: i32 = y as i32 + direction.dy;

    while posx >= 0 && posy >= 0 {
        let testx = posx as usize;
        let testy = posy as usize;
        let seat_state = seats.get(testy).map(|row| row.get(testx)).flatten();

        if seat_state.is_none() {
            return false;
        }

        let seat_position_inspected = *seat_state.unwrap();
        if seat_position_inspected != GROUND {
            return seat_position_inspected == OCCUPIED;
        }

        posx += direction.dx;
        posy += direction.dy;
    }

    return false;
}

fn is_occupied_toi(x: usize, y: usize, seats: &Vec<Vec<char>>) -> i32 {
    let occupied = seats
        .get(y)
        .map(|row| row.get(x))
        .flatten()
        .filter(|&&c| c == OCCUPIED)
        .is_some();
    return if occupied { 1 } else { 0 };
}
