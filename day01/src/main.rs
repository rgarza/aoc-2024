use std::collections::HashMap;

pub fn solve(input: &str) -> String {
    let mut left: Vec<i64> = vec![];
    let mut right: Vec<i64>  = vec![];
    input
        .lines()
        .for_each(|line| {
            let l = Some(line).unwrap_or("".into());
            let pairs = l.split_whitespace().collect::<Vec<&str>>();

            left.push(pairs[0].parse::<i64>().unwrap());
            right.push(pairs[1].parse::<i64>().unwrap());
        }
    );
    left.sort();
    right.sort();
    let sum = left.iter().zip(right.iter())
        .map(|(l, r)| {
         (r - l).abs()
    }).sum::<i64>();
    sum.to_string()

}

pub fn solve_02(input: &str) -> String {
    let mut left: Vec<i64> = vec![];
    let mut right: HashMap<i64, i64>  = HashMap::new();
    input
        .lines()
        .for_each(|line| {
            let l = Some(line).unwrap_or("".into());
            let pairs = l.split_whitespace().collect::<Vec<&str>>();

            left.push(pairs[0].parse::<i64>().unwrap());

            let r = pairs[1].parse::<i64>().unwrap();
            let current_value = right.entry(r).or_insert(0);
            *current_value += 1;
        }
    );
    left.sort();
    let mut sum: i64 = 0;
    for (_pos, l) in left.iter().enumerate() {
        let v = right.get(l);
        match v {
            Some(v) => sum += l * v,
            _ => {}
        }
    }
    sum.to_string()
}

fn main() {
    println!("Day 01/01 {:?}", solve(include_str!("./input.txt")));
    println!("Day 01/02 {:?}", solve_02(include_str!("./input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_01_01() {
        let input = include_str!("./sample.txt");
        let response = solve(input);
        assert_eq!(response, "11")
    }
    #[test]
    fn solve_01_02() {
        let input = include_str!("./sample.txt");
        let response = solve_02(input);
        assert_eq!(response, "31")
    }
}
