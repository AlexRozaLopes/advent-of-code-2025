struct Range {
    min: i32,
    max: i32,
}

impl Range {
    pub fn new(min: i32, max: i32) -> Self {
        Self { min, max }
    }
}

fn sum_invalid_ids(input: &str) -> i32 {
    input
        .split(',')
        .map(|r| {
            let mut it = r.split('-');
            let min = it.next().unwrap().parse::<i32>().unwrap();
            let max = it.next().unwrap().parse::<i32>().unwrap();
            Range::new(min, max)
        })
        .map(|range| {
            (range.min..=range.max)
                .filter(|id| is_invalid_id(id))
                .sum::<i32>()
        })
        .sum()
}

fn is_invalid_id(id: &i32) -> bool {
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

    return false;
}

#[cfg(test)]
mod tests {
    use crate::day::day_two::sum_invalid_ids;

    #[test]
    fn test_sum_invalid_ids() {
        assert_eq!(
            sum_invalid_ids(
                "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
            ),
            1227775554
        );
    }
}
