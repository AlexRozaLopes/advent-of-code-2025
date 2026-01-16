struct Range {
    min: String,
    max: String,
}

impl Range {
    pub fn new(min: String, max: String) -> Self {
        Self { min, max }
    }
}

fn sum_invalid_ids(input: &str) -> i32 {
    input
        .split(",")
        .map(|r| {
            r.split("-")
                .map(|v| v.parse::<String>().unwrap())
                .collect::<Vec<String>>()
        })
        .map(|r| Range::new(r[0].clone(), r[1].clone()))
        .map(|r| find_invalid_id(r).iter().sum::<i32>())
        .sum()
}

fn find_invalid_id(range: Range) -> Vec<i32> {
    let mut invalid_ids: Vec<i32> = Vec::new();
    if is_invalid_id(range.min.clone()) {
        invalid_ids.push(range.min.parse().unwrap());
    }

    if is_invalid_id(range.max.clone()) {
        invalid_ids.push(range.min.parse().unwrap());
    }

    invalid_ids
}

fn is_invalid_id(id: String) -> bool {
    let chars = id.chars().collect::<Vec<char>>();
    let (mut l, mut r) = (0, chars.len() - 1);
    let is_even = chars.len() % 2 == 0;
    if is_even {
        while l < r {
            if chars[l] == chars[r] {
                l += 1;
                r -= 1;
            } else {
                return false;
            }
        }
        return true
    }
    false
}

#[cfg(test)]
mod tests {
    use crate::day::day_two::sum_invalid_ids;

    #[test]
    fn test_sum_invalid_ids() {
        assert_eq!(sum_invalid_ids("11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124"), 1227775554);
    }

}