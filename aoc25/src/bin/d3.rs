fn main() {
    solve_pt1();
    solve_pt2();
}

aoc25::input!("d3");

fn solve_pt1() {
    let sum = compute_digits(2, INPUT);
    println!("{sum}");
}

fn solve_pt2() {
    let sum = compute_digits(12, INPUT);
    println!("{sum}");
}

fn compute_digits(digits: usize, input: &str) -> u64 {
    let lines = input.lines().map(|l| {
        l.chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<_>>()
    });

    let mut sum: u64 = 0;
    for line in lines {
        let mut acc = Vec::with_capacity(digits);
        let mut digits_remaining = digits;
        let mut ptr = 0;

        while digits_remaining > 0 {
            let mut i = ptr;
            let mut max_i = i;

            while i < line.len() - (digits_remaining - 1) {
                if line[max_i] < line[i] {
                    max_i = i;
                }

                i += 1;
            }

            acc.push(line[max_i]);
            ptr = max_i + 1;
            digits_remaining -= 1;
        }

        let n = concat_digits(acc);
        sum += n;
    }

    return sum;
}

fn concat_digits(digits: Vec<u32>) -> u64 {
    let mut s = String::with_capacity(digits.len());
    for d in digits {
        s.push_str(&d.to_string());
    }
    return s.parse().unwrap();
}
