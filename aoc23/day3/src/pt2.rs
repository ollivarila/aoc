pub mod solution {
    
    const EMPTY: u8 = 0x20;
    #[derive(Debug)]
    struct Schematic {
        rows: usize,
        cols: usize,
        characters: Vec<char>,
        values: Vec<Value>
    }
    
    #[derive(Debug)]
    struct Gear {
        pos: usize
    }
    
    #[derive(Debug, PartialEq)]
    struct Value {
        value: i32,
        start: usize,
        end: usize
    }
    
    impl From<&str> for Schematic {
        fn from(value: &str) -> Self {
            let cols = value.lines().collect::<Vec<&str>>();
            let characters = value.chars().filter(|c| !c.is_ascii_whitespace()).collect::<Vec<char>>(); 
            let values = find_values(&characters);
            Schematic { cols: cols.get(0).unwrap().len(), rows: cols.len(), characters, values }
        }
    }
    
    impl Schematic {
        fn value_next_to_gear(&self, gear: &Gear) -> Option<Vec<&Value>> {
            let mut result : Vec<&Value> = Vec::new();

            let left = self.characters.get(gear.pos - 1).unwrap();
            let right = self.characters.get(gear.pos + 1).unwrap();
            if left.is_digit(10) {
                let value = self.find_value(gear.pos - 1).unwrap();
                result.push(value);
            };

            if right.is_digit(10) {
                let value = self.find_value(gear.pos + 1).unwrap();
                result.push(value);
            };
            let start = gear.pos - 1 - self.cols;
            let end = gear.pos + 1 - self.cols;
            let top = self.check_range(start, end);

            let start = gear.pos - 1 + self.cols;
            let end = gear.pos + 1 + self.cols;
            let bot = self.check_range(start, end);
            if let Some(res) = top {
                res.iter().for_each(|res| result.push(*res));
            }
            if let Some(res) = bot {
                res.iter().for_each(|res| result.push(*res));
            }

            if result.len() > 0 {
                Some(result) 
            } else {
                None
            }

        }
        
        fn check_range(&self, start: usize, end: usize) -> Option<Vec<&Value>> {
            let mut result = Vec::new();
            for i in start..end + 1 {
                if let Some(c) = self.characters.get(i) {
                    if c.is_digit(10) {
                        if let Some(val)= self.find_value(i){
                            if !result.contains(&val) {
                                result.push(val);
                            }
                        };
                    }
                }
            }
            
            if result.len() > 0 {
                Some(result)
            } else {
                None
            }

        }
        
        fn find_value(&self, index: usize) -> Option<&Value> {
            for value in &self.values {
                if value.start <= index && value.end >= index {
                    return Some(value);
                }
            }
            None
        }
    }
    
    
    pub fn get_result_for(input: &str) -> i32 {
        let schematic = Schematic::from(input);
        // dbg!(&schematic);
        let gears = collect_gears(&schematic);
        // dbg!(&gears);
        // let gear = gears.get(0).unwrap();
        let mut ratios: Vec<i32> = Vec::new();
        for gear in gears {
            if let Some(result) = schematic.value_next_to_gear(&gear){
                println!("Found something!");
                dbg!(&result);
                if result.len() == 2 {
                    let g1 = result.get(0).unwrap();
                    let g2 = result.get(1).unwrap();
                    ratios.push(g1.value * g2.value);
                }
            };

        }
        
        ratios.iter().sum()
    }
    
    fn collect_gears(schematic: &Schematic) -> Vec<Gear> {
        let chars = &schematic.characters;
        let mut result = vec![];
        
        for (i, char) in chars.iter().enumerate() {
            if is_gear(char) {
                let gear = Gear {
                    pos: i
                };
                result.push(gear);
            }
        }
        result
    }
    
    fn is_gear(char: &char) -> bool {
        String::from(char.to_string()).eq("*")
    }
    
    fn find_values(chars: &Vec<char>) -> Vec<Value> {
        let mut result = Vec::new();
        let mut i = 0;
        while i < chars.len() {
            let start = i;
            let next = chars.get(i).unwrap();
            
            if next.is_digit(10) {
                let val = parse_num(&chars, start);
                i = val.end;
                result.push(val);
            }
            i += 1;
        }
        result
    }

    fn parse_num(chars: &Vec<char>, start: usize) -> Value {
        let mut str_num = String::from(chars.get(start).unwrap().to_string());
        let mut end = start;
        let empty = char::from(EMPTY);
        loop {
            let potential = chars.get(end + 1).unwrap_or(&empty);
            if !potential.is_digit(10){
                break;
            }
            str_num.push(potential.clone());
            end += 1;
        }
        let value: i32 = str_num.parse().unwrap();
        Value {
            value,
            start,
            end
        }
    }
}