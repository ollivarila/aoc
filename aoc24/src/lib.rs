use std::fmt::Display;

pub mod day_1;

pub trait Solution {
    const FOR: &'static str;
    const INPUT: &'static str;
    type SolutionOutput;
    fn solution(&self, input: &str) -> Self::SolutionOutput;
    fn display_solution(&self)
    where
        Self::SolutionOutput: Display,
    {
        let solution = self.solution(Self::INPUT);
        println!("Solution for {}: {}", Self::FOR, solution)
    }
}
