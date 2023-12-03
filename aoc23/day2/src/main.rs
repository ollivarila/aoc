use std::fs;

static TEST_GAMES: &'static str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

struct Cubes(u32, u32, u32);

fn main() {
    // let games = TEST_GAMES.split("\n").into_iter().map(Game::from).collect::<Vec<Game>>();
    let games = fs::read_to_string("input.txt").unwrap().split("\n").into_iter().map(Game::from).collect::<Vec<Game>>();
    let cubes = Cubes(12, 13, 14);
    let sum: u32 = games.iter().filter(|game| game.is_possible(&cubes)).map(|game| game.id).sum();
    println!("Pt1 sum: {}", sum);
    
    let fewest_n_cubes = games.iter().map(|game| game.fewest_n_cubes()).collect::<Vec<Cubes>>();
    let powers = fewest_n_cubes.iter().map(|cubes| cubes.0 * cubes.1 * cubes.2).collect::<Vec<u32>>();
    let sum = powers.iter().sum::<u32>();
    println!("Pt2 sum: {}", sum);
}

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

impl Game {
    fn is_possible(&self, cubes: &Cubes) -> bool {
        self.rounds.iter().all(|round| round.is_possible(cubes))
    }
    fn fewest_n_cubes(&self) -> Cubes {
        let red_max = self.find_max(|round| round.red);
        let green_max = self.find_max(|round| round.green);
        let blue_max = self.find_max(|round| round.blue);
        Cubes(red_max, green_max, blue_max)
    }
    fn find_max(&self, f: impl FnMut(&Round) -> u32) -> u32 {
        self.rounds.iter().map(f).max().unwrap()
    }
}


#[derive(Debug)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

impl Round {
    fn is_possible(&self, cubes: &Cubes) -> bool {
         !(self.red > cubes.0 || self.green > cubes.1 || self.blue > cubes.2)
    }
}

impl From<&str> for Game {
    fn from(s: &str) -> Self {
        let mut iter = s.split(":").into_iter();
        let id= iter.next().unwrap().split(" ").nth(1).unwrap().parse::<u32>().unwrap();
        let rounds = parse_rounds(iter.next().unwrap());
        Game {
            id,
            rounds,
        }
    }
}

fn parse_rounds(s: &str) -> Vec<Round> {
    let rounds = s.split(";").into_iter();
    rounds.map(parse_round).collect()
}

fn parse_round(round: &str) -> Round {
    let iter = round.trim().split(", ");
    let mut round = Round {
        red: 0,
        green: 0,
        blue: 0,
    };
    for val in iter {
        let mut iter = val.split(" ");
        let amount = iter.next().unwrap().parse::<u32>().unwrap();
        let color = iter.next().unwrap();
        match color {
            "red" => round.red = amount,
            "green" => round.green = amount,
            "blue" => round.blue = amount,
            _ => panic!("Invalid color"),
        }
    }

    round
}
