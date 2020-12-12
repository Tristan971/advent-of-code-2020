use advent_of_code_2020::common::inputs;

use crate::Turn::{LEFT, RIGHT};

#[derive(Eq, PartialEq, Copy, Clone)]
struct Direction {
    dx: i32,
    dy: i32,
}

const NORTH: Direction = Direction { dx: 0, dy: 1 };
const SOUTH: Direction = Direction { dx: 0, dy: -1 };
const WEST: Direction = Direction { dx: -1, dy: 0 };
const EAST: Direction = Direction { dx: 1, dy: 0 };

#[derive(Clone, Copy)]
enum Turn {
    LEFT,
    RIGHT,
}

fn main() {
    let lines = inputs::fread_lines("src/bin/day12/input");

    part_1(&lines);
    part_2(&lines);
}

fn part_1(lines: &Vec<String>) {
    let mut x = 0;
    let mut y = 0;
    let mut direction = EAST;

    lines.iter().for_each(|line| {
        let (action, value_str) = line.split_at(1);
        let value = value_str.parse::<i32>().unwrap();

        let (dx, dy) = match action {
            "N" | "S" | "E" | "W" => {
                let direction = dir_dxdy(action);
                (direction.dx, direction.dy)
            }
            "L" | "R" => {
                direction = dir_turn(direction, action, value);
                (0, 0)
            }
            "F" => (direction.dx, direction.dy),
            _ => panic!("Unknown action: {}", action),
        };

        x += dx * value;
        y += dy * value;
    });

    println!(
        "Final position: [{}, {}] with distance from start: {}",
        x,
        y,
        x.abs() + y.abs()
    );
}

fn part_2(lines: &Vec<String>) {
    let mut wx = 10;
    let mut wy = 1;

    let mut sx = 0;
    let mut sy = 0;

    lines.iter().for_each(|line| {
        let (action, value_str) = line.split_at(1);
        let value = value_str.parse::<i32>().unwrap();

        match action {
            // move waypoint
            "N" | "S" | "E" | "W" => {
                let direction = dir_dxdy(action);
                wx += value * direction.dx;
                wy += value * direction.dy;
            }
            // rotate waypoint vector
            "L" | "R" => {
                let turn = match action {
                    "L" => LEFT,
                    "R" => RIGHT,
                    _ => panic!("Unknown turn direction: {}", action),
                };
                let new_waypoint_pos = rotate_vector_end(wx, wy, turn, value);
                wx = new_waypoint_pos.0;
                wy = new_waypoint_pos.1;
            }
            // move ship by waypoint vector * value
            "F" => {
                sx += value * wx;
                sy += value * wy;
            }
            _ => panic!("Unknown action: {}", action),
        };
    });

    println!(
        "Final positions: Ship[{}, {}], Waypoint[{}, {}] with ship distance from start: {}",
        sx,
        sy,
        wx,
        wy,
        sx.abs() + sy.abs()
    );
}

fn dir_dxdy(direction: &str) -> Direction {
    match direction {
        "N" => NORTH,
        "S" => SOUTH,
        "E" => EAST,
        "W" => WEST,
        _ => panic!("Unknown direction: {}", direction),
    }
}

fn dir_turn(original: Direction, turn_action: &str, angle: i32) -> Direction {
    let turn = match turn_action {
        "L" => LEFT,
        "R" => RIGHT,
        _ => panic!("Unexpected turn action: {}", turn_action),
    };

    let turns = angle / 90;

    let mut current = original;
    for _ in 0..turns {
        current = dir_turn_one(current, turn);
    }

    return current;
}

fn dir_turn_one(original: Direction, turn: Turn) -> Direction {
    match turn {
        LEFT => match original {
            NORTH => WEST,
            SOUTH => EAST,
            WEST => SOUTH,
            EAST => NORTH,
            _ => panic!("Unexpected direction!"),
        },
        RIGHT => match original {
            WEST => NORTH,
            EAST => SOUTH,
            SOUTH => WEST,
            NORTH => EAST,
            _ => panic!("Unexpected direction!"),
        },
    }
}

fn rotate_vector_end(sx: i32, sy: i32, turn: Turn, angle: i32) -> (i32, i32) {
    let rclock_90deg_rots = match turn {
        LEFT => 360 - angle % 360,
        RIGHT => angle % 360,
    } / 90;

    let mut resx = sx;
    let mut resy = sy;
    for _ in 0..rclock_90deg_rots {
        let nx = resy;
        let ny = -resx;

        resx = nx;
        resy = ny;
    }

    return (resx, resy);
}
