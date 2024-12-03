pub fn solve_day_02_01(input: &str) -> String {
    let s: i64 = input
        .lines()
        .map(|report| {
            let levels: Vec<&str> = report.split_whitespace().collect();
            if is_valid_report(&levels) {
                1
            } else {
                0
            }
        })
        .sum();

    s.to_string()
}
fn is_valid_report(levels: &Vec<&str>) -> bool {
    let mut ret: i64 = 1;
    let mut going_up: Option<bool> = None;
    let mut i: usize = 0;
    if levels.len() < 2 {
        return false;
    }
    while ret == 1 && i < levels.len() - 1 {
        let current = levels[i].parse::<i64>().unwrap();
        let next = levels[i + 1].parse::<i64>().unwrap();
        if going_up.is_none() {
            if next > current {
                going_up = Some(true);
            } else {
                going_up = Some(false);
            }
        }
        i += 1;
        let diff = next - current;
        match going_up {
            Some(true) => {
                if (diff <= 0) || (diff < 1 || diff > 3) {
                    ret = 0;
                }
            }
            Some(false) => {
                if (diff >= 0) || (diff > -1 || diff < -3) {
                    ret = 0;
                }
            }
            _ => {}
        }
    }
    if ret == 1 {
        true
    } else {
        false
    }
}

pub fn solve_day_02_02(input: &str) -> String {
    let s: i64 = input
        .lines()
        .map(|report| {
            let levels: Vec<&str> = report.split_whitespace().collect();
            let mut new_levels: Vec<&str>;
            new_levels = levels.clone();
            let mut is_valid: bool = false;
            let mut current_idx: usize = 0;
            while is_valid == false && levels.len() >= current_idx {
                is_valid = is_valid_report(&new_levels);
                new_levels = levels.clone();
                if current_idx < levels.len() {
                    new_levels.remove(current_idx);
                }
                current_idx += 1;
            }
            if is_valid {
                1
            } else {
                0
            }
        })
        .sum();

    s.to_string()
}
fn main() {
    let input = include_str!("./input.txt");
    println!("Day 02/01 {:?}", solve_day_02_01(input));
    println!("Day 02/02 {:?}", solve_day_02_02(input));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve_02_01() {
        let input = include_str!("./sample.txt");
        assert_eq!("2", solve_day_02_01(input));
    }
    #[test]
    fn solve_02_02() {
        let input = include_str!("./sample.txt");
        assert_eq!("4", solve_day_02_02(input));
    }
    #[test]
    fn solve_02_01_final() {
        let input = include_str!("./input.txt");
        assert_eq!("490", solve_day_02_01(input));
    }
    #[test]
    fn solve_02_02_final() {
        let input = include_str!("./input.txt");
        assert_eq!("536", solve_day_02_02(input));
    }
}
