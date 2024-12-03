pub fn solve_day_02_01(input: &str) -> String {
    let s: i64 = input
        .lines()
        .map(|report| {
            let levels: Vec<&str> = report.split_whitespace().collect();
            let mut ret: i64 = 1;
            let mut going_up: Option<bool> = None;
            let mut i: usize = 0;
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
                        if (diff < 0) || (diff < 1 || diff > 3) {
                            ret = 0;
                        }
                    }
                    Some(false) => {
                        println!("{:?} - {:?} = {:?}", next, current, diff);
                        if (diff > 0) || (diff > -1 || diff < -3) {
                            ret = 0;
                        }
                    }
                    _ => {}
                }
            }
            if ret == 1 {
                println!("{:?}", report);
            }
            ret
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
