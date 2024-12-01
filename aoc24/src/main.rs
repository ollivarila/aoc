use clap::Parser;

#[derive(Debug, Parser)]
#[command(about = "Advent of Code 2024")]
struct Args {
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

fn main() {
    let args = Args::parse();
    aoc24::run(args.day, args.iterations);
}
