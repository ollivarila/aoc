use crate::Solution;

const INPUT: &str = include_str!("../inputs/day3_input.txt");

pub struct Part1;
pub struct Part2;

impl Solution for Part1 {
    const FOR: &'static str = "Day 3 part 1";
    const INPUT: &'static str = INPUT;
    type SolutionOutput = u64;
    fn solution(&self, input: &str) -> Self::SolutionOutput {
        MulParser::pt1(input).map(|m| m.apply()).sum()
    }
}

#[derive(Debug)]
struct MulParser<'a> {
    input: &'a str,
    mul_mode: MulMode,
}

#[derive(Debug, PartialEq)]
enum MulMode {
    Do,
    Dont,
    Disabled,
}

impl Iterator for MulParser<'_> {
    type Item = Mul;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(c) = self.peek() {
            if c == 'm' {
                let maybe_mul = self.mul();
                if maybe_mul.is_some() {
                    if matches!(self.mul_mode, MulMode::Do | MulMode::Disabled) {
                        return maybe_mul;
                    }
                }
                continue;
            }
            if c == 'd' {
                if self.mul_mode == MulMode::Disabled {
                    self.advance_n(1);
                    continue;
                }

                match self.mode() {
                    Some(next) => {
                        self.mul_mode = next;
                        continue;
                    }
                    None => {
                        continue;
                    }
                }
            }
            self.advance_n(c.len_utf8());
        }

        None
    }
}

impl MulParser<'_> {
    fn mode(&mut self) -> Option<MulMode> {
        self.ensure('d')?;
        self.ensure('o')?;

        match self.peek() {
            Some('n') => self.dont(),
            Some('(') => self.do_mode(),
            Some(_) | None => None,
        }
    }
    fn dont(&mut self) -> Option<MulMode> {
        self.ensure('n')?;
        self.ensure('\'')?;
        self.ensure('t')?;
        self.ensure('(')?;
        self.ensure(')')?;
        Some(MulMode::Dont)
    }

    fn do_mode(&mut self) -> Option<MulMode> {
        self.ensure('(')?;
        self.ensure(')')?;
        Some(MulMode::Do)
    }

    fn mul(&mut self) -> Option<Mul> {
        self.ensure('m')?;
        self.ensure('u')?;
        self.ensure('l')?;
        self.ensure('(');
        let lhs = self.num()?;
        self.ensure(',');
        let rhs = self.num()?;
        self.ensure(')')?;

        Some(Mul { lhs, rhs })
    }

    fn num(&mut self) -> Option<u64> {
        let mut num = String::new();
        while let Some(c) = self.peek() {
            if !c.is_ascii_digit() && !num.is_empty() {
                break;
            }

            num.push(c);
            self.advance_n(c.len_utf8())
        }

        num.parse().ok()
    }

    fn peek(&self) -> Option<char> {
        self.input.chars().next()
    }

    fn ensure(&mut self, what: char) -> Option<()> {
        match self.peek() {
            Some(c) if c == what => {
                self.advance_n(c.len_utf8());
                Some(())
            }
            _ => None,
        }
    }

    fn advance_n(&mut self, bytes: usize) {
        self.input = &self.input[bytes..];
    }

    fn new<'a>(input: &'a str) -> MulParser<'a> {
        MulParser {
            input,
            mul_mode: MulMode::Do,
        }
    }

    fn pt1<'a>(input: &'a str) -> MulParser<'a> {
        MulParser {
            input,
            mul_mode: MulMode::Disabled,
        }
    }
}

#[derive(Debug)]
struct Mul {
    lhs: u64,
    rhs: u64,
}

impl Mul {
    fn apply(&self) -> u64 {
        self.lhs * self.rhs
    }
}

impl Solution for Part2 {
    const FOR: &'static str = "Day 3 part 1";
    const INPUT: &'static str = INPUT;
    type SolutionOutput = u64;
    fn solution(&self, input: &str) -> Self::SolutionOutput {
        MulParser::new(input).map(|m| m.apply()).sum()
    }
}

#[cfg(test)]
mod should {

    use super::*;

    const EXAMPLE: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const EXAMPLE2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    macro_rules! ensure_muls {
        ($input:expr, $n:expr) => {
            let m = MulParser::new($input).collect::<Vec<_>>();
            assert_eq!(m.len(), $n);
        };
    }

    #[test]
    fn compute_part1_example() {
        let output = Part1.solution(EXAMPLE);
        assert_eq!(output, 161);
    }

    #[test]
    fn parse_mul() {
        ensure_muls!("mul(1,2)", 1);
        ensure_muls!("mul(96,453)mul(45,369),\nmul(403,541)what()from", 3);
        ensure_muls!(
            "mul(850,931)!where()why()~@<!{why()do() ^~%why()mul(909,10)!select(",
            2
        );
        ensure_muls!("mulselect()why()~[*!mul(530,643),#$mul(798,926)sele", 2);
        ensure_muls!("mumul(1,2)", 1);
    }

    #[test]
    fn not_parse_mul() {
        ensure_muls!("mul(?,2)", 0);
        ensure_muls!("mul()", 0);
        ensure_muls!("mul(22)", 0);
        ensure_muls!("mul ( 2 , 4 )", 0);
    }

    #[test]
    fn parse_correct_mul() {
        let m = &MulParser::new("mul(123,23)").collect::<Vec<_>>()[0];
        assert_eq!(m.lhs, 123);
        assert_eq!(m.rhs, 23);
    }

    #[test]
    fn compute_part2_example() {
        let output = Part2.solution(EXAMPLE2);
        assert_eq!(output, 48);
    }
}
