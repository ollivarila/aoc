use nom::bytes::complete::tag;
use nom::bytes::complete::take;
use nom::IResult;
use nom::{
    bytes::complete::take_until,
    character::complete::{anychar, multispace1, space1},
    sequence::{delimited, preceded, separated_pair, terminated},
};
use std::{collections::HashMap, vec};

fn main() {
    let input = include_str!("../../input.txt");
    let ans = solution(input);
    println!("{:?}", ans);
}

fn solution(input: &str) -> u128 {
    let (rules, map, starts) = parse(input);
    let instructions = Instructions {
        instructions: map,
        rules,
    };
    let all = instructions.find_targets(starts);
    lcm(&*all.into_boxed_slice())
}

fn parse(input: &str) -> (Vec<char>, HashMap<&str, (String, String)>, Vec<String>) {
    let (remaining, rules) = parse_rules(input).unwrap();

    let results: Vec<(&str, (&str, &str))> = remaining
        .lines()
        .map(|l| parse_line(l).unwrap().1)
        .collect();

    let mut map = HashMap::new();

    for (key, (left, right)) in results.clone() {
        map.insert(key, (String::from(left), String::from(right)));
    }
    let starts = map
        .iter()
        .filter(|item| item.0.ends_with("A"))
        .map(|item| item.0.to_string())
        .collect();

    (rules, map, starts)
}

fn parse_line(line: &str) -> IResult<&str, (&str, (&str, &str))> {
    separated_pair(
        take(3usize),
        delimited(space1, anychar, space1),
        parse_tuple,
    )(line)
}

fn parse_tuple(input: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(
        preceded(tag("("), take(3usize)),
        tag(", "),
        terminated(take(3usize), tag(")")),
    )(input)
}

fn parse_rules(input: &str) -> IResult<&str, Vec<char>> {
    let (r, rules) = take_until("\n\n")(input)?;
    let (r, _) = multispace1(r)?;
    Ok((r, rules.chars().collect()))
}

fn lcm(nums: &[u128]) -> u128 {
    if nums.len() == 1 {
        return nums[0];
    }

    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd(a, b)
}

fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

struct Instructions<'a> {
    instructions: HashMap<&'a str, (String, String)>,
    rules: Vec<char>,
}

impl Instructions<'_> {
    fn get_left(&self, key: &str) -> String {
        self.instructions.get(key).unwrap().0.clone()
    }
    fn get_right(&self, key: &str) -> String {
        self.instructions.get(key).unwrap().1.clone()
    }

    fn find_targets(&self, start: Vec<String>) -> Vec<u128> {
        let mut result = vec![];
        for key in start {
            let value = self.find_target(key);
            result.push(value as u128);
        }
        result
    }

    fn find_target(&self, start: String) -> u64 {
        let mut count = 1;
        let mut current = start;
        loop {
            // Should always find something
            for rule in &self.rules {
                current = match rule {
                    'L' => self.get_left(&current),
                    'R' => self.get_right(&current),
                    _ => panic!("Unexpected rule"),
                };
                if current.ends_with("Z") {
                    return count;
                }
                count += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_input1() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        let answer = solution(input);
        assert_eq!(answer, 6);
    }
}
