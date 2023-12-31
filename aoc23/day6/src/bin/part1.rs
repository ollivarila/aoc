use std::ops::Range;
use nom::IResult;
use nom::bytes::complete::tag;
use nom::multi::separated_list1;
use nom::sequence::{delimited, preceded, pair};
use nom::character::complete::{u32, newline, multispace1};
use roots::find_roots_quadratic;

fn main() {
    let input = include_str!("../../input.txt");
    let solution = solution(input);
    println!("{}", solution);
}

fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    let (r, times) = parse_time(input).unwrap();
    let (_, distances) = parse_distance(r).unwrap();
    (times, distances)
}

fn parse_time(input: &str) -> IResult<&str, Vec<u32>> {
    let (remaining, _) = tag("Time:")(input)?;
    let (remaining, parsed) = delimited(multispace1, separated_list1(multispace1, u32), newline)(remaining)?;

    Ok((remaining, parsed))
}

fn parse_distance(input: &str) -> IResult<&str, Vec<u32>> {
    preceded(pair(tag("Distance:"), multispace1), separated_list1(multispace1, u32))(input)
}

fn solution(input: &str) -> usize {
    let (times, distances) = parse(input);
    let iter = times.iter().zip(distances.iter());
    
    let mut num_ways = vec![];
    for (time, distance) in iter {
        let range = solve(time.clone() as i32, distance.clone() as i32);
        let len = range.len();
        num_ways.push(len);
    }
    num_ways.iter().product()
}

fn solve(time: i32, distance: i32) -> Range<i32> {
    let a = -1; // Always constant
    let b = time; // Total time
    let c = - distance; // Distance
    let ans = solve_quadratic(a, b, c);
    ans.0..ans.1 + 1
}

fn solve_quadratic(a: i32, b: i32, c: i32) -> (i32, i32) {
    let ans= find_roots_quadratic(a as f32, b as f32, c as f32);
    match ans {
        roots::Roots::Two(roots) => convert_roots(roots),
        _ => panic!("Did not find two roots")
    }
}

fn convert_roots(roots: [f32; 2]) -> (i32, i32) {
    // This counts for roots being exactly something
    let first = roots[0].floor() as i32 + 1;
    let second = roots[1].ceil() as i32 - 1;
    (first, second)
}


#[cfg(test)]
pub mod tests {
    use super::*;
    
    #[test]
    fn test_input() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let answer = solution(input);
        assert_eq!(answer, 288);
    }
    
}


