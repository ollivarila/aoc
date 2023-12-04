use std::fs;

mod pt1;
mod pt2;



static TEST_INPUT: &'static str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

// static TEST_INPUT: &'static str = "467..114..";

fn main() {
    // let positions = find_symbols(TEST_INPUT);
    // let rows = TEST_INPUT.split('\n').collect::<Vec<&str>>().len();
    // let columns = TEST_INPUT.split('\n').collect::<Vec<&str>>().first().unwrap().len();
    // let mut file = fs::File::open("input.txt").expect("err");
    // file.read_to_string(&mut input).expect("error reading");
    let input = fs::read_to_string("input.txt").expect("Error with opening file");
    // let schematic = pt1::solution::Schematic::from(input);
    
    // // let schematic = pt1::Schematic::from(input);
    // let mut result: Vec<&pt1::solution::Value> = Vec::new();
    // for row in schematic.rows.iter() {
    //     let res = row.scan();
    //     for value in res.iter() {
    //         result.push(value);
    //     }

    // }
    // let sum: i32 = result.iter().map(|v| v.value).sum();
    // println!("Pt1 sum: {}", sum);
    let sum = pt2::solution::get_result_for(&input);
    println!("Pt2 sum: {}", sum);
}
