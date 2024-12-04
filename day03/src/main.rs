pub fn solve_day_03_01(input: &str) -> String {
    let mut current_idx: usize = 0;
    let mut in_mul: bool = false;
    let mut curr_value_idx: usize = 0;
    let mut values: Vec<String> = vec!["".into(), "".into()];
    let mut sum: u64 = 0;
    while current_idx < input.len() {
        let c = input.chars().nth(current_idx).unwrap();
        match c {
            'm' => {
                let is_u = input.chars().nth(current_idx + 1).unwrap();
                let is_l = input.chars().nth(current_idx + 2).unwrap();
                let is_parenthisis = input.chars().nth(current_idx + 3).unwrap();
                if is_u == 'u' && is_l == 'l' && is_parenthisis == '(' {
                    in_mul = true;
                    current_idx += 4;
                } else {
                    current_idx += 1;
                }
            }
            _ => {
                if in_mul {
                    if c.is_digit(10) {
                        values[curr_value_idx].push(c);
                    } else if c == ',' {
                        if curr_value_idx == 1 {
                            in_mul = false;
                            curr_value_idx = 0;
                            values = vec!["".into(), "".into()];
                        } else {
                            curr_value_idx += 1;
                        }
                    } else if c == ')' {
                        curr_value_idx = 0;
                        let first: u64 = values[0].parse().unwrap_or(0);
                        let second: u64 = values[1].parse().unwrap_or(0);
                        sum += first * second;
                        values = vec!["".into(), "".into()];
                        in_mul = false;
                    } else {
                        curr_value_idx = 0;
                        values = vec!["".into(), "".into()];
                        in_mul = false;
                    }
                }
                current_idx += 1;
            }
        }
    }
    sum.to_string()
}
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_03_01_sample() {
        let input = include_str!("./sample.txt");
        assert_eq!(solve_day_03_01(input), "161");
    }
    #[test]
    fn day_03_01_input() {
        let input = include_str!("./input.txt");
        assert_eq!(solve_day_03_01(input), "188741603");
    }
}
