use std::ops::Range;
use std::{fs, thread, vec};

#[allow(dead_code)]
static TEST_INPUT: &'static str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    // let input = TEST_INPUT;
    println!("Parsing rules");
    let rules = parse_rules(&input);
    println!("Parsing seeds");
    let jobs = parse_seeds_pt2(&input);

    println!("Starting jobs");
    let mut results = vec![];
    
    for job in jobs {
        let result = start_job(job, rules.clone());
        results.push(result);
    }
    
    let min = results.iter().min().unwrap();
    println!("Min: {}", min)
    

}

fn start_job(job: SeedRange, rules: Vec<RuleChain>) -> u64 {
    let subjobs = create_subjobs(job, 6);
    
    println!("Starting job");
    let mut handles = vec![];
    for job in subjobs {
        let cp = rules.clone();
        let handle = thread::spawn(move || {
            let min = job.into_iter().map(|val| transform(val, &cp)).min().unwrap();
            min
        });
        handles.push(handle);
    }
    
    let mut results = vec![];
    println!("Waiting for {} subjobs to finish", handles.len());
    for handle in handles {
        if let Ok(result) = handle.join() {
            results.push(result);
        } else {
            panic!("Thread failed");
        }

    }

    println!("Done with job");
    results.iter().min().unwrap().to_owned()
}

fn create_subjobs(job: SeedRange, n: u64) -> Vec<Range<u64>> {
    let section = (job.end - job.start) / n;
    let mut result = vec![];
    let mut current_start = job.start;
    while current_start + section < job.end {
        let range = current_start..current_start + section;
        result.push(range);
        current_start += section;
    }
    let remaining_range = current_start..job.end;
    result.push(remaining_range);
    result
}

#[test]
fn subjobs() {
    let seed_range = SeedRange {
        start: 0,
        end: 20
    };
    let subjobs = create_subjobs(seed_range, 12);
    assert_eq!(subjobs.len(), 20);
    let seed_range = SeedRange {
        start: 0,
        end: 120 
    };
    let subjobs = create_subjobs(seed_range, 12);
    assert_eq!(subjobs.len(), 12);
    let seed_range = SeedRange {
        start: 0,
        end: 135 
    };
    let subjobs = create_subjobs(seed_range, 12);
    assert_eq!(subjobs.len(), 13);
    let last = subjobs.last().unwrap().to_owned();
    assert_eq!(last.last().unwrap(), 134)
}

fn parse_seeds_pt2(input: &str) -> Vec<SeedRange> {
    let mut iter = input.lines().into_iter();
    let seeds = iter.next().unwrap();
    let seed_nums: Vec<u64> = seeds.split(": ").last().unwrap().split(" ").map(|num| num.parse::<u64>().unwrap()).collect();
    let tuples = seed_nums.chunks(2);
    let mut result = vec![];
    for tuple in tuples.into_iter() {
        let range_start = tuple[0];
        let range_len: u64 = tuple[1];

        let range = SeedRange {
            start: range_start,
            end: range_start + range_len
        };
        result.push(range);
    }
    result
}

#[derive(Debug)]
struct SeedRange {
    start: u64,
    end: u64
}

#[allow(dead_code)]
fn parse_seeds_pt1(input: &str) -> Vec<u64> {
    let mut iter = input.lines().into_iter();
    let seeds = iter.next().unwrap();
    seeds.split(": ").last().unwrap().split(" ").map(|num| num.parse::<u64>().unwrap()).collect()
}

fn parse_rules(s: &str) -> Vec<RuleChain> {
    let mut iter = s.lines().into_iter();
    iter.next();  // Consume seeds
    iter.next(); // Consume empty line

    let mut buf: Vec<&str> = vec![];
    let mut rules: Vec<RuleChain> = vec![];
    for line in iter {
        let c = line.chars().nth(0).unwrap_or(' ');
        if c.is_digit(10) {
            buf.push(line);
            continue;
        }

        if c.is_ascii_whitespace() {
            let chain = RuleChain::from(buf.clone());
            buf.clear();
            rules.push(chain);
        }
    }

    if buf.len() > 0 {
        let thing = RuleChain::from(buf);
        rules.push(thing);
    }
    rules
}


#[derive(Debug, Clone, Copy)]
struct Rule {
    src_start: u64,
    dst_start: u64,
    range_len: u64,
}

impl Rule {
    fn apply(&self, val: u64) -> u64 {
        if val >= self.src_start && val < self.src_start + self.range_len {
            val - self.src_start + self.dst_start
        } else {
            val
        }
    }
}

impl From<&str> for Rule {
    fn from(value: &str) -> Self {
        let mut iter = value.split(" ").into_iter();
        let dst_start = iter.next().unwrap().parse().unwrap();
        let src_start = iter.next().unwrap().parse().unwrap();
        let range_len = iter.next().unwrap().parse().unwrap();
        Rule {
            dst_start,
            src_start,
            range_len
        }
    }
}

#[test]
fn rule(){
    let rule = Rule::from("50 98 2");
    assert_eq!(rule.apply(98), 50);
    assert_eq!(rule.apply(99), 51);
    assert_eq!(rule.apply(100), 100);
    
    let rule = Rule::from("52 50 48");
    assert_eq!(rule.apply(79), 81);
    assert_eq!(rule.apply(1), 1);
}

#[derive(Debug, Clone)]
struct RuleChain {
    rules: Vec<Rule>
}

impl RuleChain {
    fn apply(&self, val: u64) -> u64 {
        let mut current = val;
        for rule in &self.rules {
            let applied = rule.apply(val);
            if applied != current {
               return applied;
            }
            current = applied;

        }
        current
    }
}

impl From<Vec<&str>> for RuleChain {
    fn from(value: Vec<&str>) -> RuleChain {
        let rules = value.iter().map(|s| Rule::from(*s)).collect();
        RuleChain {
            rules
        }
    }
}

#[test]
fn rule_combined() {
    let rules = vec![Rule::from("50 98 2"), Rule::from("52 50 48")];
    let chain: RuleChain = RuleChain { rules };

    assert_eq!(chain.apply(98), 50);
    assert_eq!(chain.apply(79), 81);
    assert_eq!(chain.apply(100), 100);
    assert_eq!(chain.apply(1), 1);
}

fn transform(val: u64, rules: &Vec<RuleChain>) -> u64 {
    let mut current = val;
    for rule in rules {
        current = rule.apply(current);
    }
    current
}