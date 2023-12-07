use nom::IResult;
use nom::bytes::complete::{tag, take_until};
use nom::multi::{separated_list1};
use nom::sequence::{delimited, preceded};
use nom::character::complete::{u32, newline, multispace1};

fn main() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let res = parse(input);
        dbg!(res);
}

fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    let (_, times) = parse_time(input).unwrap();
    let (_, distances) = parse_distance(input).unwrap();
    (times, distances)
}

fn parse_time(input: &str) -> IResult<&str, Vec<u32>> {
    let (remaining, parsed) = tag("Time:")(input)?;
    let (remaining, parsed) = delimited(multispace1, separated_list1(multispace1, u32), newline)(remaining)?;

    Ok((remaining, parsed))
}

fn parse_distance(input: &str) -> IResult<&str, Vec<u32>> {
    preceded(tag("Distance:"), separated_list1(multispace1, u32))(input)
}

fn solution(input: &str) -> u64 {
    todo!()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    
    #[test]
    fn test_input() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let answer = solution(input);
        assert_eq!(answer, 288);
    }
    
}


