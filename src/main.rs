use crate::day::{day_one, day_two};

mod day;

fn main() {
    println!(
        "Day 1: Secret Entrance: PART ONE - ANSWER {}",
        day_one::password(include_str!("data/day-one"))
    );

    println!(
            "Day 1: Secret Entrance: PART TWO - ANSWER {}",
        day_one::new_password(include_str!("data/day-one"))
    );

    println!(
        "Day 1: Secret Entrance: PART TWO - ANSWER {}",
        day_one::password_method_0x434c49434b(include_str!("data/day-one"))
    );


    println!(
        "Day 2: Gift Shop: PART ONE - ANSWER {}",
        day_two::sum_invalid_ids(include_str!("data/day-two"))
    );

    println!(
        "Day 2: Gift Shop: PART TWO - ANSWER {}",
        day_two::sum_invalid_ids_part_two(include_str!("data/day-two"))
    );
}
