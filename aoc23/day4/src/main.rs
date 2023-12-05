use std::collections::{HashSet, HashMap};
use std::fs;
use std::convert::TryFrom;

static TEST_INPUT: &'static str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

fn input(path: Option<&str>) -> String {
    if let Some(path) = path {
        fs::read_to_string(path).unwrap()
    } else {
        TEST_INPUT.to_string()
    }
}

fn main() {
    let mut i = 0;
    let input = input(Some("input.txt"));
    let cards = input.lines().map(|e| {
        let mut card = Card::from(e);
        card.index = i;
        i += 1;
        card
    }).collect::<Vec<Card>>();
    let total: u32 = cards.iter().map(wins_and_points).sum();
    let processed = cards.iter().map(|card| process_card(card, &cards, &mut HashMap::new())).collect::<Vec<Vec<&Card>>>();
    println!("Total points: {}", total);
    let total: usize = processed.iter().map(|cards| cards.len()).sum();
    dbg!(total);
}

fn process_card<'a>(card: &'a Card, cards: &'a Vec<Card>, cache: &mut HashMap<usize, usize>) -> Vec<&'a Card> {
    // Memoization
    let wins = if let Some(wins) = cache.get(&card.index) {
        wins.clone()
    } else {
        let n_wins = usize::try_from(get_n_wins(card)).unwrap();
        cache.insert(card.index, n_wins);
        n_wins.clone()
    };

    // let wins = usize::try_from(get_n_wins(card)).unwrap();
    let mut result: Vec<&Card> = Vec::new();
    result.push(card);
    let card_idx = card.index;
    for i in card_idx + 1..card_idx + wins + 1 {
        let next = process_card(cards.get(i).unwrap(), cards, cache);
        next.iter().for_each(|card| result.push(card));
    }

    result
}


#[allow(dead_code)]
#[derive(Debug, PartialEq)]
struct Card {
    index: usize,
    winning_numbers: HashSet<u32>,
    numbers: HashSet<u32>,
}

impl From<&str> for Card {
    fn from(value: &str) -> Self {
        let nums = value.split(":").nth(1).expect("numbers").trim();
        let mut nums_iter = nums.split("|").into_iter();
        let l_nums = nums_iter.next().unwrap().trim();
        let r_nums = nums_iter.next().unwrap().trim();
        
        let mut winning_numbers = HashSet::new();
        l_nums.split(" ").for_each(|n| {
            if let Ok(num) = n.parse::<u32>() {
                winning_numbers.insert(num);

            };
        });

        let mut numbers = HashSet::new();
        r_nums.split(" ").for_each(|n| {
            if let Ok(num) = n.parse::<u32>() {
                numbers.insert(num);
            };
        });
        
        // Check if parsing was correct
        // assert_eq!(winning_numbers.len(), 10);
        // assert_eq!(numbers.len(), 25);
        

        Card {
            index: 0,
            winning_numbers,
            numbers
        }
    }
    
}

fn wins_and_points(card: &Card) -> u32 {
    calc_points(get_n_wins(card))
}
 
fn get_n_wins(card: &Card) -> u32 {
    let mut matches = 0;
    
    for number in card.numbers.iter() {
        if card.winning_numbers.contains(number) {
            matches += 1;
        }
    }
    
    matches
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

 