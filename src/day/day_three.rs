pub fn total_output_joltage(input: &str) -> i32 {
    input
        .lines()
        .map(|line| largest_joltage(line).unwrap())
        .sum()
}

fn largest_joltage(input: &str) -> Option<i32> {
    let digits: Vec<i32> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();

    let mut max_joltage = -1;

    for i in 0..digits.len() {
        for j in (i + 1)..digits.len() {
            let joltage = digits[i] * 10 + digits[j];
            max_joltage = max_joltage.max(joltage);
        }
    }

    if max_joltage == -1 {
        None
    } else {
        Some(max_joltage)
    }
}

#[cfg(test)]
mod tests {
    use crate::day::day_three::total_output_joltage;

    #[test]
    fn test_total_output_joltage() {
        assert_eq!(
            total_output_joltage(
                "987654321111111
811111111111119
234234234234278
818181911112111
"
            ),
            357
        );
    }
}
