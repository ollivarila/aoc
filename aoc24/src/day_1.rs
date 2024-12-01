use crate::Solution;
use anyhow::Context;
use std::collections::HashMap;

const INPUT: &str = include_str!("../inputs/day1_input.txt");

pub struct Part1;

impl Solution for Part1 {
    const FOR: &'static str = "Day 1 part 1";
    const INPUT: &'static str = INPUT;
    type SolutionOutput = u64;
    fn solution(&self, input: &str) -> Self::SolutionOutput {
        let (mut left, mut right) = parse_input(input);
        left.sort();
        right.sort();

        let mut total = 0;

        for (left, right) in left.iter().zip(right.iter()) {
            total += (left - right).abs();
        }

        total as u64
    }
}

pub struct Part2;

impl Solution for Part2 {
    const FOR: &'static str = "Day 1 part 2";
    const INPUT: &'static str = INPUT;
    type SolutionOutput = u64;
    fn solution(&self, input: &str) -> Self::SolutionOutput {
        let (left, right) = parse_input(input);
        let freqs = compute_freqs(&right);

        let mut total = 0;

        for key in left {
            total += key * freqs.get(&key).unwrap_or(&0);
        }

        total as u64
    }
}

pub fn parse_input(input: &str) -> (Vec<i64>, Vec<i64>) {
    let mut left = vec![];
    let mut right = vec![];
    for line in input.lines() {
        let mut iter = line.split_whitespace();
        let left_num = iter
            .next()
            .context("left num")
            .and_then(|n| n.parse::<i64>().context("convert num"))
            .expect("left num");
        left.push(left_num);

        let right_num = iter
            .next()
            .context("right num")
            .and_then(|n| n.parse::<i64>().context("convert num"))
            .expect("right num");
        right.push(right_num);
    }

    (left, right)
}

fn compute_freqs(values: &Vec<i64>) -> HashMap<i64, i64> {
    let mut freqs = HashMap::new();

    for value in values {
        match freqs.get(value) {
            Some(v) => freqs.insert(*value, v + 1),
            None => freqs.insert(*value, 1),
        };
    }

    freqs
}

#[cfg(test)]
mod should {

    use super::*;
    const EXAMPLE: &str = "3   4
4   3
2   5
1   3
3   9
3   3
";

    #[test]
    fn compute_p1_example() {
        assert_eq!(Part1.solution(EXAMPLE), 11);
    }

    #[test]
    fn compute_p2_example() {
        assert_eq!(Part2.solution(EXAMPLE), 31)
    }
}
