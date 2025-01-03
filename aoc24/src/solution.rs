use indicatif::{ProgressBar, ProgressStyle};
use owo_colors::OwoColorize;

use crate::bench::{Bench, BenchConfig};
use std::{fmt::Display, time::Duration};

pub trait Solution {
    const FOR: &'static str;
    const INPUT: &'static str;
    type SolutionOutput;
    fn solution(&self, input: &str) -> Self::SolutionOutput;
    fn display_solution(&self, config: &BenchConfig)
    where
        Self::SolutionOutput: Display,
    {
        let for_italic = Self::FOR;
        let day_part = for_italic.blue();

        let bar = ProgressBar::new(config.iterations as u64);
        let style = ProgressStyle::default_bar()
            .template("\t{msg} [{bar:40.cyan/blue}] {pos}/{len} (eta {eta})")
            .unwrap()
            .progress_chars("=> ");

        bar.set_style(style);
        bar.set_message(format!("Running {day_part}"));

        let report = self.run_bench(config, &bar);
        let solution_bold = "Solution".bold();
        let solution = solution_bold.purple();
        let result = report.output.bright_green();

        let time = time_taken(report.took);

        let time_bold = time.bold();

        bar.finish_and_clear();

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
