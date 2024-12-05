pub fn can_go_down(_x: usize, y: usize, current_len: usize, _width: usize, height: usize) -> bool {
    let missing_chars_len = 4 - current_len;
    if y + missing_chars_len >= height {
        return false;
    } else {
        return true;
    }
}

pub fn can_go_right(x: usize, _y: usize, current_len: usize, width: usize, _height: usize) -> bool {
    let missing_chars_len = 4 - current_len;
    if x + missing_chars_len >= width {
        return false;
    } else {
        return true;
    }
}

pub fn can_go_up(_x: usize, y: usize, current_len: usize, _width: usize, _height: usize) -> bool {
    let missing_chars_len = 4 - current_len;
    if y < missing_chars_len {
        return false;
    } else {
        return true;
    }
}
pub fn can_go_left(x: usize, _y: usize, current_len: usize, _width: usize, _height: usize) -> bool {
    let missing_chars_len = 4 - current_len;
    if x < missing_chars_len {
        return false;
    } else {
        return true;
    }
}
#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}
fn process_char(data: &Vec<Vec<char>>, x: usize, y: usize, current: &String, direction: Direction) -> i32 {
    if current.len() >= 4 || x >= data[0].len() || y >= data.len()  {
        return 0;
    }
    let mut new_current: String = current.clone();
    let new_char = data[y][x];
    let valid = match current.len() {
        0 if new_char == 'X' => true,
        1 if new_char == 'M' => true,
        2 if new_char == 'A' => true,
        3 if new_char == 'S' => true,
        _ => false,
    };
    if !valid {
        return 0;
    }
    new_current.push(new_char);

    if new_current == "XMAS" {
        println!("XMAS, Direction {:?}, position {:?}-{:?}", direction, y, x);
        return 1;
    }
    let width = data[0].len();
    let height = data.len();
    match direction {
        Direction::Up => {
            let can = can_go_up(x, y, new_current.len(), width, height);
            if can {
                return process_char(data, x, y - 1, &new_current, direction);
            }
        }
        Direction::Down => {
            let can = can_go_down(x, y, new_current.len(), width, height);
            if can {
                 return process_char(data, x, y + 1, &new_current, direction);
            }
        }
        Direction::UpLeft => {
            let up = can_go_up(x, y, new_current.len(), width, height);
            let left = can_go_left(x, y, new_current.len(), width, height);
            if up && left {
                return process_char(data, x - 1, y - 1, &new_current, direction);
            }
        }
        Direction::UpRight => {
            let up = can_go_up(x, y, new_current.len(), width, height);
            let right = can_go_right(x, y, new_current.len(), width, height);
            if up && right {
                return process_char(data, x + 1, y - 1, &new_current, direction);
            }
        }
        Direction::DownLeft => {
            let down = can_go_down(x, y, new_current.len(), width, height);
            let left = can_go_left(x, y, new_current.len(), width, height);
            if down && left {
                return process_char(data, x - 1, y + 1, &new_current, direction);
            }
        }
        Direction::DownRight => {
            let down = can_go_down(x, y, new_current.len(), width, height);
            let right = can_go_right(x, y, new_current.len(), width, height);
            if down && right {
                return process_char(data, x + 1 , y + 1, &new_current, direction);
            }
        }
        Direction::Left => {
            let can = can_go_left(x, y, new_current.len(), width, height);
            if can {
                return process_char(data, x - 1, y, &new_current, direction);
            }
        }
        Direction::Right => {
            //println!("{:?} Position {:?}-{:?}", new_current, y, x);
            let can = can_go_right(x, y, new_current.len(), width, height);
            if can {
                return process_char(data, x + 1, y, &new_current, direction);
            }
        }
    };
    0
}
pub fn solve_day_04_01(input: &str) -> String {
    let mut data: Vec<Vec<char>> = vec![];
    let empty: String = "".into();
    input.lines().for_each(|line| {
        data.push(line.chars().collect());
    });

    let mut total: i32 = 0;
    for y in 0..data[0].len() {
        for x in 0..data[0].len() {
            if data[y][x] == 'X' {
                total += process_char(&data, x, y, &empty, Direction::Left);
                total += process_char(&data, x, y, &empty, Direction::Right);
                total += process_char(&data, x, y, &empty, Direction::Down);
                total += process_char(&data, x, y, &empty, Direction::Up);
                total += process_char(&data, x, y, &empty, Direction::DownRight);
                total += process_char(&data, x, y, &empty, Direction::DownLeft);
                total += process_char(&data, x, y, &empty, Direction::UpRight);
                total += process_char(&data, x, y, &empty, Direction::UpLeft);
            }
        }
    }

    total.to_string()
}
fn main() {
    let input = include_str!("./input.txt");
    println!("Day 04 part 01{:?}", solve_day_04_01(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_04_01() {
        let sample = include_str!("./sample.txt");
        assert_eq!(solve_day_04_01(sample), "18");
    }

    #[test]
    fn can_go_right_test() {
        assert_eq!(can_go_right(8, 0, 0, 10, 10), false);
        assert_eq!(can_go_right(8, 0, 2, 10, 10), false);
        assert_eq!(can_go_right(8, 0, 3, 10, 10), true);
    }
    #[test]
    fn can_go_left_test() {
        assert_eq!(can_go_left(0, 0, 0, 10, 10), false);
        assert_eq!(can_go_left(0, 0, 3, 10, 10), false);
        assert_eq!(can_go_left(1, 0, 3, 10, 10), true);
        assert_eq!(can_go_left(8, 0, 0, 10, 10), true);
        assert_eq!(can_go_left(3, 0, 0, 10, 10), false);
        assert_eq!(can_go_left(4, 0, 0, 10, 10), true);
    }
}
