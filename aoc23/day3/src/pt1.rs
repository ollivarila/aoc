pub mod solution {

const EMPTY: u8 = 0x20;
    
#[derive(Debug)]
pub struct Schematic {
    pub rows: Vec<Row>
}

impl From<String> for Schematic {
    fn from(value: String) -> Self {
        let lines = value.lines().collect::<Vec<&str>>();
        let mut rows = Vec::new();
        for (i, line) in value.lines().enumerate() {
            let start_i = if i == 0 {
                0
            } else {
                i - 1
            };
            let top = *lines.get(start_i).unwrap_or(&"");
            let mid = line;
            let bot = *lines.get(i + 1).unwrap_or(&"");
            let row = create_row(top, mid, bot, i);
            rows.push(row);
        }

        Schematic {
            rows 
        }

    }
}

fn create_row(top: &str, mid: &str, bot: &str, row_num: usize) -> Row {
    let orig = String::from(mid);
    let mask = create_mask(top, mid, bot);
    let values = find_values(mid);
    
    Row {
        orig,
        mask,
        values,
        row_num
    }
}


fn create_mask(top: &str, mid: &str, bot: &str) -> String {
    let mut mask = String::from(mid);
    for (i, c) in top.chars().enumerate() {
        if is_symbol(&c) {
            mask.replace_range(i..i+1, &c.to_string());
        }
    }
    for (i, c) in bot.chars().enumerate() {
        if is_symbol(&c) {
            mask.replace_range(i..i+1, &c.to_string());
        }
    }
    
    mask
}


fn is_symbol(c: &char) -> bool {
    let s = String::from(c.clone());
    // let symbols = vec!["*", "#", "+", "$", "@", "/", "=", "&", "%", "-"];
    s.eq("*") || s.eq("#") || s.eq("+") || s.eq("$") || s.eq("@") || s.eq("/") || s.eq("=") || s.eq("&") || s.eq("%") || s.eq("-")
}

#[derive(Debug)]
pub struct Row {
    orig: String,
    mask: String,
    values: Vec<Value>,
    row_num: usize
}

impl Row {
    pub fn scan(&self) -> Vec<&Value> {
        let mut result = Vec::new();
        let empty = char::from(EMPTY);
        for value in self.values.iter() {
            let mask = self.mask.chars().collect::<Vec<char>>();
            
            let start = if value.start == 0 {
                0
            } else {
                value.start - 1
            };
            let end = value.end + 2;

            for i in start..end {
                let mask_c = mask.get(i).unwrap_or(&empty);
                if is_symbol(mask_c) {
                    result.push(value);
                }
            }
        }
        
        result 
    }
}

#[derive(Debug)]
pub struct Value {
    pub value: i32,
    start: usize,
    end: usize,
}



fn find_values(input: &str) -> Vec<Value> {
    let mut result = Vec::new();
    let chars = input.chars().collect::<Vec<char>>();
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

