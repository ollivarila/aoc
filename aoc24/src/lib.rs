use owo_colors::OwoColorize;
use std::{
    fmt::Display,
    time::{Duration, Instant},
};

mod day_1;

pub trait Solution {
    const FOR: &'static str;
    const INPUT: &'static str;
    type SolutionOutput;
    fn solution(&self, input: &str) -> Self::SolutionOutput;
    fn display_solution(&self, config: &BenchConfig)
    where
        Self::SolutionOutput: Display,
    {
        let report = self.run_bench(config);
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

fn prompt() {
    let content = "Aoc 2024".bold();
    println!("\n\t{content}\n");
}

pub fn run(day: Option<u8>, iterations: usize) {
    let config = BenchConfig::new(iterations);
    prompt();
    if let None = day {
        return run_all(&config);
    }

    match day.unwrap() {
        1 => {
            day_1::Part1.display_solution(&config);
            day_1::Part2.display_solution(&config);
        }
        n if n > 24 => panic!("There are only 24 days in aoc?"),
        n => unimplemented!("Day {n}"),
    }
}

fn run_all(config: &BenchConfig) {
    day_1::Part1.display_solution(&config);
    day_1::Part2.display_solution(&config);
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
    fn run_bench(&self, config: &BenchConfig) -> Report<T>;
}

impl<T> Bench<T::SolutionOutput> for T
where
    T: Solution + ?Sized,
{
    fn run_bench(&self, config: &BenchConfig) -> Report<T::SolutionOutput> {
        let mut all_durations = Vec::with_capacity(config.iterations);
        let mut output = None;
        for _ in 0..config.iterations {
            let start = Instant::now();
            output = Some(self.solution(Self::INPUT));
            let after = Instant::now();
            let took = after - start;
            all_durations.push(took);
        }

        let took = all_durations.iter().sum::<Duration>() / config.iterations as u32;

        Report {
            took,
            output: output.unwrap(),
        }
    }
}

pub struct BenchConfig {
    iterations: usize,
}

impl BenchConfig {
    pub fn new(iterations: usize) -> Self {
        Self { iterations }
    }
}
