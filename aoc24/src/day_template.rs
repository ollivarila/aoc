use crate::Solution;

const INPUT: &str = include_str!("../inputs/day2_input.txt");

pub struct Part1;
pub struct Part2;

impl Solution for Part1 {
    const FOR: &'static str = todo!();
    const INPUT: &'static str = INPUT;
    type SolutionOutput = u64;
    fn solution(&self, input: &str) -> Self::SolutionOutput {
        todo!()
    }
}

impl Solution for Part2 {
    const FOR: &'static str = todo!();
    const INPUT: &'static str = INPUT;
    type SolutionOutput = u64;
    fn solution(&self, input: &str) -> Self::SolutionOutput {
        todo!()
    }
}

#[cfg(test)]
mod should {

    use super::*;
    const EXAMPLE: &str = todo!();

    #[test]
    fn compute_part1_example() {
        let output = Part1.solution(EXAMPLE);
        assert_eq!(output, todo!());
    }

    #[test]
    fn compute_part2_example() {
        let output = Part2.solution(EXAMPLE);
        assert_eq!(output, todo!());
    }
}
