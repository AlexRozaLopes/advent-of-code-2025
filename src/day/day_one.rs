pub fn password(input: &str) -> i32 {
    let mut count = 0;
    let mut point: i32 = 50;

    for pass in input.lines() {
        if pass.is_empty() {
            continue;
        }

        let rotation = pass.chars().next().unwrap();
        let number: i32 = pass[1..].parse().unwrap();

        match rotation {
            'R' => point += number,
            'L' => point -= number,
            _ => {}
        }

        point = ((point % 100) + 100) % 100;

        if point == 0 {
            count += 1;
        }
    }

    count
}
pub fn new_password(input: &str) -> i32 {
    let mut count = 0;
    let mut point: i32 = 50;

    for pass in input.lines() {
        if pass.is_empty() { continue; }

        let rotation = pass.chars().next().unwrap();
        let number: i32 = pass[1..].parse().unwrap();
        let start = point;

        match rotation {
            'R' => {
                count += (start + number) / 100;
                point = start + number;
            }
            'L' => {
                let k0 = if start == 0 { 100 } else { start };
                if number >= k0 {
                    count += 1 + (number - k0) / 100;
                }
                point = start - number;
            }
            _ => {}
        }

        point = ((point % 100) + 100) % 100;

    }

    count
}

pub fn password_method_0x434c49434b(input: &str) -> i32 {
    let mut count: i32 = 0;
    let mut pos: i32 = 50; // posição inicial

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() { continue; }

        let dir = line.chars().next().expect("linha inválida");
        let distance: i32 = line[1..].parse().expect("número inválido");

        // mover um passo por clique
        for _ in 0..distance {
            match dir {
                'L' | 'l' => {
                    pos -= 1;
                    if pos < 0 { pos = 99; }
                }
                'R' | 'r' => {
                    pos += 1;
                    if pos > 99 { pos = 0; }
                }
                _ => panic!("direção inválida: {}", dir),
            }
            if pos == 0 { count += 1; }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let sample = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";
        assert_eq!(password_method_0x434c49434b(sample), 6);
    }
}


