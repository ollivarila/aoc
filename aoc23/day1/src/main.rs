use std::env;
use std::fs;

fn main() {
    let mut args = env::args().collect::<Vec<String>>();
    let _program = args.remove(0);
    let file_path = args.pop().unwrap_or("".to_string());
    let solution = pt2(file_path);
    println!("Solution: {}", solution);
}

static TEST_INPUT: &'static str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

fn pt1(file_path: String) -> i32 {
    if file_path == "" {
        let iter= TEST_INPUT.split("\n");
        let sum: i32 = iter.map(parse_item).sum();
        return sum;
    }
    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");
    let iter = contents.split("\n");
    let sum: i32 = iter.map(parse_item).sum();
    return sum;
}

fn parse_item(item: &str) -> i32 {
    let mut digits = item.chars().filter(|c| c.is_digit(10)).collect::<Vec<char>>();
    if digits.len() < 2 {
        let digit = digits.pop().unwrap();
        let s = digit.to_string() + digit.to_string().as_str();
        return s.parse().unwrap();
    }

    let result = digits[0].to_string() + digits.pop().unwrap().to_string().as_str();
    result.parse().unwrap()
}

fn pt2(file_path: String) -> i32 {
    println!("file_path: {}", file_path);
    if file_path == "" {
        let iter = TEST_INPUT.split("\n");
        let sum: i32 = iter.map(parse_item2).sum();
        return sum;
    }
    
    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");
    let iter = contents.split("\n");
    let sum: i32 = iter.map(parse_item2).sum();
    return sum;
}

fn parse_item2(item: &str) -> i32 {
    let left = find_left(item);
    let right = find_right(item);
    let num = left.to_string() + right.to_string().as_str();
    num.parse().unwrap()
}

fn find_right(s: &str) -> i32 {
    let mut left = s.len() - 1;
    let mut right = s.len();
    while right > 0 {
        let search = &s[left..right];
        if let Some(n) = found(search) {
            return n;
        }
        if left == 0 {
            right -= 1;
            left = right - 1;
        } else {
            left -= 1;
        }
    }

    unreachable!("Should find something");
}
fn find_left(s: &str) -> i32 {
    let mut left = 0;
    let mut right = 1;
    while left < s.len() {
        if let Some(n) = found(&s[left..right]) {
            return n;
        }
        if right == s.len() {
            left += 1;
            right = left + 1;
        } else {
            right += 1;
        }
    }

    unreachable!("Should find something");
}

fn found(s: &str) -> Option<i32> {
    if let Some(num) = s.parse::<i32>().ok() {
        return Some(num);
    }
    
    match s {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    }
}