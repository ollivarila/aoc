use std::time::{Duration, Instant};

use crate::solution::Solution;

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
