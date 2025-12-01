use crate::day::day_one;

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
}
