use std::{str::FromStr, vec};

fn main() {
    solve_pt1();
    solve_pt2();
}

aoc25::input!("d6");

type Problem = (Vec<u64>, Op);

#[derive(Clone, Copy, Debug)]
enum Op {
    Sum,
    Prod,
}

impl FromStr for Op {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "*" => Ok(Op::Prod),
            "+" => Ok(Op::Sum),
            _ => Err("invalid character"),
        }
    }
}

impl Op {
    fn parse(c: char) -> Option<Op> {
        match c {
            '*' => Some(Op::Prod),
            '+' => Some(Op::Sum),
            _ => None,
        }
    }
}

fn parse_input_pt1(input: &str) -> Vec<Problem> {
    let count = input.lines().count();
    let lines = input.lines().enumerate();
    let mut num_lines: Vec<Vec<u64>> = Vec::with_capacity(count - 2);
    let mut ops: Vec<Op> = vec![];

    for (i, line) in lines {
        if i == count - 1 {
            ops = line
                .split_whitespace()
                .map(|op| op.parse().unwrap())
                .collect();
        } else {
            let nums = line
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            num_lines.push(nums);
        }
    }

    let end = num_lines[0].len();

    let mut problems = vec![];

    for i in 0..end {
        let mut nums = Vec::with_capacity(num_lines.len());

        for row in num_lines.iter() {
            nums.push(row[i]);
        }
        problems.push((nums, ops[i]))
    }

    problems
}

fn solve_pt1() {
    let problems = parse_input_pt1(INPUT);
    let sum = compute_problems(problems);
    println!("{sum}");
}

fn compute_problems(problems: Vec<Problem>) -> u64 {
    let mut sum = 0;
    for (nums, op) in problems {
        let result = nums.iter().skip(1).fold(nums[0], |a, b| match op {
            Op::Prod => a * *b,
            Op::Sum => a + *b,
        });

        sum += result;
    }

    sum
}

fn parse_input_pt2(input: &str) -> Vec<Problem> {
    let input = transpose(input);
    let count = input[0].len();
    let mut problems = vec![];

    let mut acc = vec![];
    #[allow(unused_assignments)]
    let mut op_opt = None;
    for row in input {
        if let Some(op) = Op::parse(row[count - 1]) {
            op_opt = Some(op);
        }
        let s = row.iter().take(count - 1).collect::<String>();
        if s.trim().is_empty() {
            problems.push((acc, op_opt.take().unwrap()));
            acc = vec![];
            continue;
        }

        let n: u64 = s.trim().parse().unwrap();
        acc.push(n);
    }

    problems.push((acc, op_opt.unwrap()));

    problems
}

fn transpose(input: &str) -> Vec<Vec<char>> {
    let original: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let rows = original.len();
    let cols = original[0].len();
    let empty = vec!['0'; rows];
    let mut transposed: Vec<Vec<char>> = vec![empty; cols];

    for row in 0..rows {
        for col in 0..cols {
            transposed[col][row] = original[row][col];
        }
    }

    transposed
}

fn solve_pt2() {
    let problems = parse_input_pt2(INPUT);
    let sum = compute_problems(problems);

    println!("{sum}");
}
