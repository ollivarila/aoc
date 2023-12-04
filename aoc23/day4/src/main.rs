use std::collections::HashSet;


fn main() {
    println!("Hello, world!");
    let res = calc_points(1);
    dbg!(res);
}

#[allow(dead_code)]
struct Card {
    winning_numbers: HashSet<u32>,
    numbers: HashSet<u32>,
}

impl From<&str> for Card {
    fn from(_value: &str) -> Self {
        let winning_numbers = HashSet::new();
        let numbers = HashSet::new();
        Card {
            winning_numbers,
            numbers
        };
        todo!();
    }
    
}

fn get_n_wins(card: &Card) -> u32 {
    todo!();
}

fn calc_points(n_matches: u32) -> u32 {
    if n_matches == 0 {
        return 0;
    }

    if n_matches == 1 {
        1
    } else {
       2 * calc_points(n_matches - 1)
    }
}

#[test]
fn points() {
    assert_eq!(calc_points(4), 8);
    assert_eq!(calc_points(1), 1);
    assert_eq!(calc_points(0), 0);
}

 