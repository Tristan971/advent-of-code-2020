use std::collections::HashMap;

struct Appearance {
    last: i32,
    penultimate: i32,
}

fn main() {
    //let example = Vec::from([0, 3, 6]);
    //part_1(&example, 2020);

    let starting_numbers = Vec::from([2, 0, 6, 12, 1, 3]);
    run_for(&starting_numbers, 2020);
    run_for(&starting_numbers, 30000000);
}

fn run_for(starting_numbers: &Vec<i32>, max_turn: usize) {
    let mut number_to_appearances: HashMap<i32, Appearance> = HashMap::new();

    for i in 1..starting_numbers.len() + 1 {
        number_to_appearances.insert(
            starting_numbers[i - 1],
            Appearance {
                last: i as i32,
                penultimate: -1,
            },
        );
    }

    let mut turn = starting_numbers.len() + 1;
    let mut last_spoken: i32 = starting_numbers[starting_numbers.len() - 1];

    while turn <= max_turn {
        let last_spoken_appearances = number_to_appearances.get(&last_spoken).unwrap();

        let spoken_this_turn = if last_spoken_appearances.penultimate == -1 {
            0
        } else {
            last_spoken_appearances.last - last_spoken_appearances.penultimate
        };
        last_spoken = spoken_this_turn;
        //println!("Turn {}: {}", turn, spoken_this_turn);

        let appearance = number_to_appearances
            .entry(spoken_this_turn)
            .or_insert_with(|| Appearance {
                last: -1,
                penultimate: -1,
            });
        appearance.penultimate = appearance.last;
        appearance.last = turn as i32;

        turn += 1;
    }

    println!("Spoken at turn {}: {}", turn - 1, last_spoken)
}
