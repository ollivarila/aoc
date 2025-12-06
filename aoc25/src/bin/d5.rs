use std::{collections::HashSet, ops::RangeInclusive};

fn main() {
    solve_pt1();
    solve_pt2();
}

aoc25::input!("d5");

fn parse_input(input: &str) -> (Vec<RangeInclusive<u64>>, Vec<u64>) {
    let lines = input.lines();
    let mut ranges: Vec<RangeInclusive<u64>> = vec![];
    let mut nums = vec![];
    let mut ranges_done = false;
    for line in lines {
        if line.is_empty() && !ranges_done {
            ranges_done = true;
            continue;
        }

        if !ranges_done {
            let mut split = line.split('-');
            let lhs: u64 = split.next().unwrap().parse().unwrap();
            let rhs = split.next().unwrap().parse().unwrap();
            assert!(split.next().is_none());
            ranges.push(lhs..=rhs);
        } else {
            nums.push(line.trim().parse().unwrap());
        }
    }

    (ranges, nums)
}

fn solve_pt1() {
    let (ranges, xs) = parse_input(INPUT);

    let mut fresh = HashSet::new();

    for x in xs {
        for range in ranges.iter() {
            if range.contains(&x) {
                fresh.insert(x);
            }
        }
    }

    let count = fresh.iter().count();
    println!("{count}");
}

fn solve_pt2() {
    let (ranges, _) = parse_input(INPUT);
    let mut merged = merge_ranges(ranges);
    let mut stop = false;
    while !stop {
        let len = merged.len();
        merged = merge_ranges(merged);
        merged.reverse();
        stop = len == merged.len();
    }

    let count = merged
        .into_iter()
        .fold(0, |count, range| count + range.count());

    println!("{count}");
}

fn merge_ranges(ranges: Vec<RangeInclusive<u64>>) -> Vec<RangeInclusive<u64>> {
    let mut merged_ranges: Vec<RangeInclusive<u64>> = Vec::new();

    for candidate in ranges {
        let start = candidate.start();
        let end = candidate.end();
        let found = {
            merged_ranges.iter_mut().find(|r| {
                (*start >= r.start() - 1 && *start <= r.end() + 1)
                    || (end >= r.start() && end <= r.end())
            })
        };
        match found {
            Some(range) => {
                *range = merge_into(candidate, range.clone());
            }
            None => {
                merged_ranges.push(candidate);
            }
        }
    }

    merged_ranges
}

fn merge_into(what: RangeInclusive<u64>, to: RangeInclusive<u64>) -> RangeInclusive<u64> {
    let start = *what.start().min(to.start());
    let end = *what.end().max(to.end());
    start..=end
}

#[cfg(test)]
mod tests {
    use crate::{merge_into, merge_ranges};

    #[test]
    fn test_merge_into() {
        assert_eq!(merge_into(0..=1, 0..=10), 0..=10);
        assert_eq!(merge_into(5..=100, 0..=10), 0..=100);
        assert_eq!(merge_into(5..=10, 11..=20), 5..=20);
    }

    #[test]
    fn test_merge_ranges() {
        let ranges = vec![1..=10, 11..=20];
        let result = merge_ranges(ranges);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], 1..=20);

        let ranges = vec![1..=10, 11..=20, 30..=40];
        let result = merge_ranges(ranges);
        assert_eq!(result.len(), 2);

        let ranges = vec![1..=10, 11..=20, 30..=40, 30..=40, 35..=40];
        let result = merge_ranges(ranges);
        assert_eq!(result.len(), 2);

        let ranges = vec![30..=40, 41..=42];
        let result = merge_ranges(ranges);
        assert_eq!(result.len(), 1);

        let ranges = vec![30..=40, 25..=39];
        let result = merge_ranges(ranges);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].start(), &25);

        let ranges = vec![30..=40, 35..=45];
        let result = merge_ranges(ranges);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].start(), &30);

        let ranges = vec![10..=100, 30..=40];
        let result = merge_ranges(ranges);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].start(), &10);
        assert_eq!(result[0].end(), &100);
    }
}
