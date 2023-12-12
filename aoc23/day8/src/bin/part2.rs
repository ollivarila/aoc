use indicatif::ProgressIterator;
use std::collections::HashMap;
use nom::{sequence::{separated_pair, delimited, preceded, terminated}, character::complete::{multispace1, space1, anychar}, bytes::complete::take_until};
use nom::IResult;
use nom::bytes::complete::take;
use nom::bytes::complete::tag;

fn main() {
    let input = include_str!("../../input2.txt");
    let ans = solution(input);
    println!("{:?}", ans);
}

fn solution(input: &str) -> u64 {
    let (rules, map, starts)= parse(input);
    let instructions = Instructions { instructions: map, rules };
    instructions.find_target(starts)
}

fn parse(input: &str) -> (Vec<char>, HashMap<&str, (String, String)>, Vec<String>) {
    let (remaining, rules ) = parse_rules(input).unwrap();

    let results: Vec<(&str, (&str, &str))> = remaining.lines().map(|l| {
        parse_line(l).unwrap().1
    }).collect();
    
    let mut map = HashMap::new();
    
    for (key, (left, right)) in results.clone() {
        map.insert(key, (String::from(left), String::from(right)));
    }
    let starts = map
        .iter()
        .filter(|item| item.0.ends_with("A"))
        .map(|item| item.0.to_string()).collect();
    
    (rules, map, starts)
}

fn parse_line(line: &str) -> IResult<&str, (&str, (&str, &str))> {
    separated_pair(take(3usize), delimited(space1, anychar, space1), parse_tuple)(line)
}

fn parse_tuple(input: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(preceded(tag("("), take(3usize)), tag(", "), terminated(take(3usize), tag(")")))(input)
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
    
    fn find_target(&self, starts: Vec<String>) -> u64 {
        self.find_target_rec(starts, 1)
    }
    fn find_target_rec(&self, start: Vec<String>, count: u64) -> u64 {
        let mut currents = start.clone();
        let mut count = count;
        loop {
            for rule in &self.rules {
                let thing: Vec<String> = currents.iter().map(|val| {
                    match rule {
                        'L' => {
                            self.get_left(val)
                        },
                        'R' => {
                            self.get_right(val)
                        },
                        _ => panic!("Unknown rule")
                    }
                }).collect();
                
                let at_end = thing.iter().all(|s| s.ends_with("Z"));
                if at_end {
                    return count;
                }
                else {
                    currents = thing;
                    count += 1;
                }
            };
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
