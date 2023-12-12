use nom::bytes::complete::tag;
use nom::bytes::complete::take;
use nom::IResult;
use nom::{
    bytes::complete::take_until,
    character::complete::{alpha1, anychar, multispace1, space1},
    sequence::{delimited, preceded, separated_pair, terminated},
};
use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input.txt");
    let ans = solution(input);
    println!("{:?}", ans);
}

fn solution(input: &str) -> u64 {
    let (rules, map, (start, target)) = parse(input);
    dbg!(&rules.len(), &start, &target);
    let instructions = Instructions {
        instructions: map,
        rules,
    };
    instructions.find_target("AAA", "ZZZ")
}

fn parse(input: &str) -> (Vec<char>, HashMap<&str, (String, String)>, (String, String)) {
    let (remaining, rules) = parse_rules(input).unwrap();

    let results: Vec<(&str, (&str, &str))> = remaining
        .lines()
        .map(|l| parse_line(l).unwrap().1)
        .collect();

    let mut map = HashMap::new();

    for (key, (left, right)) in results.clone() {
        map.insert(key, (String::from(left), String::from(right)));
    }
    let start = results.first().unwrap().0;
    let target = results.last().unwrap().0;

    (rules, map, (start.to_string(), target.to_string()))
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
        preceded(tag("("), alpha1),
        tag(", "),
        terminated(alpha1, tag(")")),
    )(input)
}

fn parse_rules(input: &str) -> IResult<&str, Vec<char>> {
    let (r, rules) = take_until("\n\n")(input)?;
    let (r, _) = multispace1(r)?;
    Ok((r, rules.chars().collect()))
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

    fn find_target(&self, start: &str, target: &str) -> u64 {
        self.find_target_rec(start.to_string(), target, 1)
    }
    fn find_target_rec(&self, start: String, target: &str, count: u64) -> u64 {
        let mut current = start.to_string();
        let mut count = count;
        for rule in &self.rules {
            match rule {
                'L' => {
                    current = self.get_left(&current);
                }
                'R' => {
                    current = self.get_right(&current);
                }
                _ => panic!("Unknown rule"),
            }
            if current == target {
                return count;
            }
            count += 1;
        }
        self.find_target_rec(current, target, count)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_input1() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        let answer = solution(input);
        assert_eq!(answer, 2);
    }

    #[test]
    fn test_input2() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        let answer = solution(input);
        assert_eq!(answer, 6)
    }
}
