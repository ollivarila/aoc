use std::time::{Duration, Instant};

use indicatif::ProgressBar;

use crate::solution::Solution;

#[derive(Debug, Clone)]
pub struct Report<T> {
    pub took: Duration,
    pub output: T,
}

pub trait Bench<T> {
    fn run_bench(&self, config: &BenchConfig, bar: &ProgressBar) -> Report<T>;
}

impl<T> Bench<T::SolutionOutput> for T
where
    T: Solution + ?Sized,
{
    // TODO: Progress bar that incements on each iteration
    // indicatif has some kind of Iterator progress bar ttrait thingy
    fn run_bench(&self, config: &BenchConfig, bar: &ProgressBar) -> Report<T::SolutionOutput> {
        let mut all_durations = Vec::with_capacity(config.iterations);
        let mut output = None;
        for _ in 0..config.iterations {
            let start = Instant::now();
            output = Some(self.solution(Self::INPUT));
            let after = Instant::now();
            let took = after - start;
            all_durations.push(took);
            bar.inc(1);
        }

        let took = all_durations.iter().sum::<Duration>() / config.iterations as u32;

        bar.set_position(config.iterations as u64);

        Report {
            took,
            output: output.unwrap(),
        }
    }
}

pub struct BenchConfig {
    pub iterations: usize,
}

impl BenchConfig {
    pub fn new(iterations: usize) -> Self {
        Self { iterations }
    }
}
