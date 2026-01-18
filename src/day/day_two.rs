struct Range {
    min: i64,
    max: i64,
}

impl Range {
    pub fn new(min: i64, max: i64) -> Self {
        Self { min, max }
    }
}

pub fn sum_invalid_ids(input: &str) -> i64 {
    input
        .split(',')
        .map(|r| {
            let mut it = r.split('-');
            let min = it.next().unwrap().trim().parse::<i64>().unwrap();
            let max = it.next().unwrap().trim().parse::<i64>().unwrap();
            Range::new(min, max)
        })
        .map(|range| {
            (range.min..=range.max)
                .filter(|id| is_invalid_id(id))
                .sum::<i64>()
        })
        .sum()
}

pub fn sum_invalid_ids_part_two(input: &str) -> i64 {
    input
        .split(',')
        .map(|r| {
            let mut it = r.split('-');
            let min = it.next().unwrap().trim().parse::<i64>().unwrap();
            let max = it.next().unwrap().trim().parse::<i64>().unwrap();
            Range::new(min, max)
        })
        .map(|range| {
            (range.min..=range.max)
                .filter(|id| is_repeated_pattern(*id))
                .sum::<i64>()
        })
        .sum()
}

fn is_invalid_id(id: &i64) -> bool {
    let s = id.to_string();
    let chars = s.chars().collect::<Vec<_>>();
    let mid = chars.len() / 2;

    if chars.len() % 2 != 0 {
        return false;
    }

    let first_half = &chars[..mid];
    let second_half = &chars[mid..];

    if first_half == second_half {
        return true;
    }

    false
}

fn is_repeated_pattern(num: i64) -> bool {
    let s = num.to_string();
    let len = s.len();

    for i in 2..=len {
        if len % i != 0 {
            continue;
        }

        let part_len = len / i;
        let part = &s[..part_len];

        if part.repeat(i) == s {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use crate::day::day_two::{sum_invalid_ids, sum_invalid_ids_part_two};

    #[test]
    fn test_sum_invalid_ids() {
        assert_eq!(
            sum_invalid_ids(
                "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
            ),
            1227775554
        );
    }

    #[test]
    fn test_sum_invalid_ids_part_two() {
        assert_eq!(
            sum_invalid_ids_part_two(
                "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
            ),
            4174379265
        );
    }
}
