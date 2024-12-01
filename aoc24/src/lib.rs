use owo_colors::OwoColorize;
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
        let result = self.solution(Self::INPUT);
        let solution_bold = "Solution".bold();
        let solution = solution_bold.purple();
        let for_italic = Self::FOR;
        let day_part = for_italic.blue();
        let result = result.bright_green();

        println!("\t{solution} for {day_part}: {result}\n")
    }
}
