pub fn solve_day_02_01(input: &str) -> String {
    let s: i64 = input
        .lines()
        .map(|report| {
            let levels: Vec<&str> = report.split_whitespace().collect();
            let mut valid: bool = true;
            let mut going_up: Option<bool> = None;
            for l in 0..levels.len() - 1 {
                let current = levels[l].parse::<i64>().unwrap();
                let next = levels[l + 1].parse::<i64>().unwrap();
                let diff = next - current;
                if going_up.is_none() {
                    if diff > 0 {
                        going_up = Some(true)
                    } else {
                        going_up = Some(false);
                    }
                }
                if going_up == Some(true) {
                    if diff < 1 || diff > 3 {
                        valid = false;
                    }
                } else {
                    if diff > -1 || diff < -3 {
                        valid = false;
                    }
                }
            }
            if valid {
                println!("{:?}", levels);
                1i64
            } else {
                0i64
            }
        })
        .sum();

    s.to_string()
}
fn main() {
    let input = include_str!("./input.txt");
    println!("Day 02/01 {:?}", solve_day_02_01(input));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve_02_01() {
        let input = include_str!("./sample.txt");
        assert_eq!("2", solve_day_02_01(input));
    }
}
