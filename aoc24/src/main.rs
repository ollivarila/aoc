use aoc24::day_1;
use aoc24::Solution;
use owo_colors::OwoColorize;

fn main() {
    prompt();
    day_1::Part1.display_solution();
    day_1::Part2.display_solution();
}

fn prompt() {
    let content = "Aoc 2024".bold();
    println!("\n\t{content}\n");
}
