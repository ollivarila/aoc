use crate::{BenchConfig, Solution};
use clap::Parser;
use owo_colors::OwoColorize;

#[derive(Debug, Parser)]
#[command(about = "Advent of Code 2024")]
pub struct Args {
    #[arg(short, long, help = "Day to run, run all days if not specified")]
    day: Option<u8>,
    #[arg(
        short,
        long,
        help = "Number of iterations for benchmarks",
        default_value_t = 100
    )]
    iterations: usize,
}

fn prompt() {
    let content = "Aoc 2024".bold();
    println!("\n\t{content}\n");
}

macro_rules! day {
    ($mod:tt, $config:expr) => {{
        crate::$mod::Part1.display_solution(&$config);
        crate::$mod::Part2.display_solution(&$config);
    }};
}

pub fn run(args: Args) {
    let day = args.day;
    let iterations = args.iterations;
    let config = BenchConfig::new(iterations);
    prompt();
    if day.is_none() {
        return run_all(&config);
    }

    match day.unwrap() {
        1 => day!(day_1, config),
        2 => day!(day_2, config),
        3 => day!(day_3, config),
        4 => day!(day_4, config),
        n if n > 24 => panic!("There are only 24 days in aoc?"),
        n => unimplemented!("Day {n}"),
    }
}

fn run_all(config: &BenchConfig) {
    day!(day_1, config);
    day!(day_2, config);
    day!(day_3, config);
    day!(day_4, config);
}
