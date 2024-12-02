use crate::Solution;

const INPUT: &str = include_str!("../inputs/day2_input.txt");

pub struct Part1;
pub struct Part2;

impl Solution for Part1 {
    const FOR: &'static str = "Day 2 part 1";
    const INPUT: &'static str = INPUT;
    type SolutionOutput = usize;
    fn solution(&self, input: &str) -> Self::SolutionOutput {
        let reports = parse_input(input);

        reports
            .into_iter()
            .map(|r| is_safe(r, 0))
            .filter(|is_safe| *is_safe)
            .count()
    }
}

fn parse_input(input: &str) -> Vec<Report> {
    let mut acc = vec![];
    for line in input.lines() {
        let mut nums = vec![];
        for value in line.split_whitespace() {
            nums.push(value.parse::<i64>().expect("number"))
        }
        acc.push(Report(nums))
    }

    acc
}

fn is_safe(Report(mut nums): Report, dampen_amount: usize) -> bool {
    is_increasing(&nums, dampen_amount) || {
        nums.reverse();
        is_increasing(&nums, dampen_amount)
    }
}

impl Solution for Part2 {
    const FOR: &'static str = "Day 2 part 2";
    const INPUT: &'static str = INPUT;
    type SolutionOutput = usize;
    fn solution(&self, input: &str) -> Self::SolutionOutput {
        let reports = parse_input(input);

        reports
            .into_iter()
            .map(|r| is_safe(r, 1))
            .filter(|is_safe| *is_safe)
            .count()
    }
}

fn is_increasing(slice: &[i64], max: usize) -> bool {
    if slice.len() <= 1 {
        return true;
    }

    let mut cur = slice.first().unwrap();

    for (i, v) in slice.iter().skip(1).enumerate() {
        if cur >= v {
            return {
                let mut v = slice.to_vec();
                v.remove(i);
                max != 0 && is_increasing(v.as_slice(), max - 1)
            } || {
                let mut v = slice.to_vec();
                v.remove(i + 1);
                max != 0 && is_increasing(v.as_slice(), max - 1)
            };
        }

        if (cur - v).abs() > 3 {
            return {
                let mut v = slice.to_vec();
                v.remove(i);
                max != 0 && is_increasing(v.as_slice(), max - 1)
            } || {
                let mut v = slice.to_vec();
                v.remove(i + 1);
                max != 0 && is_increasing(v.as_slice(), max - 1)
            };
        }
        cur = v;
    }

    true
}

#[derive(Debug, Clone)]
struct Report(Vec<i64>);

#[cfg(test)]
mod should {

    use super::*;
    const EXAMPLE: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn compute_part1_example() {
        let output = Part1.solution(EXAMPLE);
        assert_eq!(output, 2);
    }

    #[test]
    fn compute_part2_example() {
        let output = Part2.solution(EXAMPLE);
        assert_eq!(output, 4);
    }

    #[test]
    fn compute_increasing() {
        assert!(is_increasing(&[1, 2, 3, 4], 0));
        assert!(!is_increasing(&[6, 4, 2, 1], 0));
        assert!(is_increasing(&[1, 2, 3], 1));
        assert!(is_increasing(&[1, 6, 3], 1));
        assert!(!is_increasing(&[1, 6, 12], 1));
    }

    macro_rules! r {
        ($($x:expr),*) => {
            Report(vec![$($x),*])
        };
    }

    macro_rules! safe {
        ($($x:expr),*) => {
            assert!(is_safe(r!($($x),*), 1));
        };
    }

    macro_rules! not_safe {
        ($($x:expr),*) => {
            assert!(!is_safe(r!($($x),*), 1));
        };
    }

    #[test]
    fn be_safe() {
        let r = r!(6, 4, 2, 1);
        assert!(is_safe(r, 0));

        let r = r!(1, 2, 7, 8, 9);
        assert!(!is_safe(r, 0));

        safe!(1, 6, 3, 4);
        not_safe!(1, 12, 3, 2);
        safe!(12, 3, 2, 1);
        safe!(1, 2, 2, 5);
        safe!(1, 2, 4, 5);
        safe!(2, 5, 6, 8, 6);
        safe!(55, 54, 53, 52, 50, 49, 46, 42);
    }
}
