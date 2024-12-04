pub fn solve_day_03(input: &str, allow_enable: bool) -> String {
    let mut current_idx: usize = 0;
    let mut in_mul: bool = false;
    let mut curr_value_idx: usize = 0;
    let mut values: Vec<String> = vec!["".into(), "".into()];
    let mut sum: u64 = 0;
    let mut enable_mul: bool = true;
    while current_idx < input.len() {
        let c = input.chars().nth(current_idx).unwrap();
        match c {
            'd' if allow_enable => {
                let is_o = input.chars().nth(current_idx + 1).unwrap();
                let is_n = input.chars().nth(current_idx + 2).unwrap();
                let is_quote = input.chars().nth(current_idx + 3).unwrap();
                let is_t = input.chars().nth(current_idx + 4).unwrap();
                let is_po = input.chars().nth(current_idx + 5).unwrap();
                let is_pc = input.chars().nth(current_idx + 6).unwrap();

                if is_o == 'o'
                    && is_n == 'n'
                    && is_quote == '\''
                    && is_t == 't'
                    && is_po == '('
                    && is_pc == ')'
                {
                    enable_mul = false;
                    current_idx += 7;
                } else {
                    let is_po = input.chars().nth(current_idx + 2).unwrap();
                    let is_pc = input.chars().nth(current_idx + 3).unwrap();
                    if is_o == 'o' && is_po == '(' && is_pc == ')' {
                        enable_mul = true;
                        current_idx += 4;
                    }
                }
            }
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
                        if enable_mul {
                            let first: u64 = values[0].parse().unwrap_or(0);
                            let second: u64 = values[1].parse().unwrap_or(0);
                            sum += first * second;
                        }
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
    let input = include_str!("./input.txt");
    println!("Day 03.01 {:?}", solve_day_03(input, false));
    println!("Day 03.02 {:?}", solve_day_03(input, true));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_03_01_sample() {
        let input = include_str!("./sample.txt");
        assert_eq!(solve_day_03(input, false), "161");
    }
    #[test]
    fn day_03_02_sample() {
        let input = include_str!("./sample.txt");
        assert_eq!(solve_day_03(input, true), "48");
    }
    #[test]
    fn day_03_01_input() {
        let input = include_str!("./input.txt");
        assert_eq!(solve_day_03(input, false), "188741603");
    }
    #[test]
    fn day_03_02_input() {
        let input = include_str!("./input.txt");
        assert_eq!(solve_day_03(input, true), "67269798");
    }
}
