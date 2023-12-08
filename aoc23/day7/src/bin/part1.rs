use std::{cmp::Ordering, collections::HashMap};

fn main() {
    let input = include_str!("../../input.txt");
    let ans = solution(input);
    println!("{}", ans);
}

fn solution(input: &str) -> u64 {
    let mut hands = parse(input);
    hands.sort_by(|a, b| b.cmp(a)); // Sort in reverse order
    
    let mut result = vec![];
    for (i, hand) in hands.iter().enumerate() {
        let value = hand.bid * (i + 1) as u64;
        result.push(value);
    }
    
    result.iter().sum()
}

fn parse(input: &str) -> Vec<Hand> {
    input.lines().map(Hand::from).collect()
}

#[derive(Debug, PartialEq, PartialOrd, Eq)]
struct Hand {
    kind: HandKind,
    cards: Vec<u8>,
    bid: u64 
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let mut iter = value.split(" ");
        let cards = iter.next().unwrap();
        let bid = iter.next().unwrap().parse().unwrap();
        let cards = parse_cards(cards);
        let kind = HandKind::from(&cards);
        
        Hand {
            kind,
            bid,
            cards
        }
    }
}

fn parse_cards(cards: &str) -> Vec<u8> {
    let mut result = vec![];
    for c in cards.chars() {
        if let Some(n) = c.to_digit(10) {
            result.push(n as u8)
        } else {
            let n = match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
                'T' => 10,
                _ => panic!("Found unknown character {}", c)
            };
            result.push(n as u8);
        }
    }
    result
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.kind.cmp(&other.kind) {
            Ordering::Equal => cmp_cards(self, other),
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less
        }
    }
}

fn cmp_cards(a: &Hand, b: &Hand) -> Ordering {
    let iter = a.cards.iter().zip(b.cards.iter());
    for (a_card, b_card) in iter {
        if a_card == b_card {
            continue; // Same card so skip
        }
        
        if a_card < b_card {
            return Ordering::Greater;
        }
        
        if b_card < a_card {
            return Ordering::Less;
        }
    }
    panic!("No cards to compare");
}

#[derive(Debug, PartialOrd, PartialEq, Eq, Ord)]
enum HandKind {
    FiveOK,
    FourOK,
    FullHouse,
    ThreeOK,
    TwoPair,
    Pair,
    High,
}

impl From<&Vec<u8>> for HandKind {
    fn from(value: &Vec<u8>) -> Self {
        let mut counts: HashMap<u8, u8> = HashMap::new();
        for v in value {
            if let Some(cur) = counts.get(v) {
                counts.insert(*v, cur + 1);
            } else {
               counts.insert(*v, 1);
            }
        }
        let max_dup= counts.values().max().unwrap();
        let min_dup = counts.values().min().unwrap();
        match max_dup {
            5 => HandKind::FiveOK,
            4 => HandKind::FourOK,
            3 => if *min_dup == 2 as u8 { // ThreeOK or Full House
                HandKind::FullHouse
            } else {
                HandKind::ThreeOK
            }, 
            2 => {
                let pairs_count = counts.values().filter(|val| *val == &2).count();
                if pairs_count == 2 {
                    HandKind::TwoPair
                } else {
                    HandKind::Pair
                }
            },
            1 => HandKind::High,
            _ => panic!("Got unexpected amount of same rank: {}", max_dup)
        }
    }
    
}

#[cfg(test)]
mod tests {
    
    use std::cmp::Ordering;

    use super::*;

    #[test]
    fn test_input() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let answer = solution(input);
        assert_eq!(answer, 6440)
    }
    
    #[test]
    fn test_compare() {
        let hk = HandKind::FiveOK;
        let hk2 = HandKind::FullHouse;
        assert_eq!(hk.cmp(&hk2), Ordering::Less);
        let hk = HandKind::FiveOK;
        let hk2 = HandKind::FiveOK;
        assert_eq!(hk.cmp(&hk2), Ordering::Equal);
        let hk = HandKind::High;
        let hk2 = HandKind::FiveOK;
        assert_eq!(hk.cmp(&hk2), Ordering::Greater);
    }
    
    #[test]
    fn test_handkinds() {
        assert_eq!(HandKind::from(&parse_cards("11111")), HandKind::FiveOK);
        assert_eq!(HandKind::from(&parse_cards("22333")), HandKind::FullHouse);
        assert_eq!(HandKind::from(&parse_cards("12JJJ")), HandKind::ThreeOK);
        assert_eq!(HandKind::from(&parse_cards("12345")), HandKind::High);
        assert_eq!(HandKind::from(&parse_cards("11234")), HandKind::Pair);
    }

}