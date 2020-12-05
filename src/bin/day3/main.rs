use advent_of_code_2020::common::inputs;

fn main() {
    let lines = inputs::fread_lines("src/bin/day3/input");

    let slope_3_1 = count_trees_for_slope(3, 1, &lines);
    println!("Part 1: {}", slope_3_1);

    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let combined_slopes = slopes
        .iter()
        .map(|(dx, dy)| count_trees_for_slope(*dx, *dy, &lines))
        .fold(1, |s1, s2| s1 * s2);
    println!("Part 2: {}", combined_slopes);
}

fn count_trees_for_slope(dx: i64, dy: i64, map: &Vec<String>) -> i64 {
    let line_len = map[0].len();

    let mut encountered = 0;

    let mut x = 0;
    let mut y = 0;

    while y < map.len() {
        if map[y].chars().nth(x).unwrap() == '#' {
            encountered += 1;
        }
        x = (x + dx as usize) % line_len;
        y += dy as usize;
    }

    println!(
        "Slope [dx: {}, dy: {}] encountered {} trees",
        dx, dy, encountered
    );
    return encountered;
}
