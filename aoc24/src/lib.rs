use owo_colors::OwoColorize;
use std::{
    fmt::Display,
    time::{Duration, Instant},
};

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
        let report = self.run_bench();
        let solution_bold = "Solution".bold();
        let solution = solution_bold.purple();
        let for_italic = Self::FOR;
        let day_part = for_italic.blue();
        let result = report.output.bright_green();

        let time = time_taken(report.took);

        let time_bold = time.bold();

        println!("\t{solution} for {day_part}: {result} took {time_bold}\n")
    }
}

fn time_taken(duration: Duration) -> String {
    if duration.as_millis() > 10 {
        format!("{} ms", duration.as_millis())
    } else {
        format!("{} μs", duration.as_micros())
    }
}

#[derive(Debug, Clone)]
pub struct Report<T> {
    pub took: Duration,
    pub output: T,
}

pub trait Bench<T> {
    fn run_bench(&self) -> Report<T>;
}

const ITERATIONS: usize = 1000;

impl<T> Bench<T::SolutionOutput> for T
where
    T: Solution + ?Sized,
{
    fn run_bench(&self) -> Report<T::SolutionOutput> {
        let mut all_durations = Vec::with_capacity(ITERATIONS);
        let mut output = None;
        for _ in 0..ITERATIONS {
            let start = Instant::now();
            output = Some(self.solution(Self::INPUT));
            let after = Instant::now();
            let took = after - start;
            all_durations.push(took);
        }

        let took = all_durations.iter().sum::<Duration>() / ITERATIONS as u32;

        Report {
            took,
            output: output.unwrap(),
        }
    }
}
