use std::collections::HashSet;
// Looks horrible but it works

fn main() {
    solve_pt1();
    solve_pt2();
}

const TEST: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
aoc25::input!("d2");

type IdRange = (&'static str, &'static str);

fn parse_input() -> impl Iterator<Item = IdRange> {
    INPUT
        .split(",")
        .map(|item| item.split("-"))
        .map(|mut items| (items.next().unwrap(), items.next().unwrap()))
}

fn solve_pt1() {
    let mut acc = HashSet::new();
    for r in parse_input() {
        compute_invalid_ids(r, 2, &mut acc);
    }
    let solution = acc.iter().sum::<u64>();
    println!("{solution}");
}

fn solve_pt2() {
    let mut acc = HashSet::new();
    for r in parse_input() {
        let len = r.0.len().max(r.1.len());
        for i in 2..=len {
            compute_invalid_ids(r, i, &mut acc);
        }
    }
    let solution = acc.iter().sum::<u64>();
    println!("{solution}");
}

fn compute_invalid_ids((lhs, rhs): IdRange, chunks: usize, acc: &mut HashSet<u64>) {
    if rhs.len() < chunks {
        return;
    }

    let lhs = if lhs.len() % chunks != 0 {
        truncate_up(lhs)
    } else {
        lhs.to_string()
    };
    let rhs = if rhs.len() % chunks != 0 {
        truncate_down(rhs)
    } else {
        rhs.to_string()
    };

    let digits = lhs.len();
    let rhs = rhs.parse().unwrap();
    let lhs = lhs.parse().unwrap();

    let multiplier = multiplier(digits, chunks);
    let mut n = start_from(digits, chunks);
    loop {
        let next = multiplier * n;
        if next < lhs {
            n += 1;
            continue;
        }

        if next > rhs {
            break;
        }
        acc.insert(next);
        n += 1;
    }
}

fn start_from(digits: usize, chunks: usize) -> u64 {
    let mut s = "1".to_string();
    let chunk_size = digits / chunks;
    for _ in 1..chunk_size {
        s.push('0');
    }

    s.parse().unwrap()
}

fn multiplier(digits: usize, chunks: usize) -> u64 {
    if digits == 2 {
        return 11;
    }
    if digits == chunks {
        return "1".repeat(digits).parse().unwrap();
    }
    let mut s = String::with_capacity(digits);

    let chunk_size = digits / chunks;
    let mut remaining = chunk_size * (chunks - 1);

    while remaining > 1 {
        s.push('1');
        remaining -= 1;
        for _ in 1..chunk_size {
            s.push('0');
            if remaining == 1 {
                break;
            }
            remaining -= 1;
        }
    }
    s.push('1');

    s.parse().unwrap()
}

fn truncate_up(start: &str) -> String {
    let mut s = String::with_capacity(start.len() + 1);
    s.push('1');
    for _ in 0..start.len() {
        s.push('0');
    }
    s
}

fn truncate_down(start: &str) -> String {
    "9".repeat(start.len() - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiplier() {
        assert_eq!(multiplier(2, 2), 11);
        assert_eq!(multiplier(4, 2), 101);
        assert_eq!(multiplier(10, 5), 101010101);
        assert_eq!(multiplier(10, 2), 100001);
    }
}
