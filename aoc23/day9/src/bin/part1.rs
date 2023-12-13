fn main() {
    let input = include_str!("../../input.txt");
    let ans = solution(input);
    println!("{}", ans);
}

fn solution(input: &str) -> i64 {
    let parsed = parse(input);
    let derived = derive_all(parsed);
    derived.iter().map(extrapolate).sum()
}

fn parse(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .into_iter()
        .map(|line| {
            line.split(" ")
                .into_iter()
                .map(|itm| itm.parse().unwrap())
                .collect()
        })
        .collect()
}

fn extrapolate(input: &DerivationResult) -> i64 {
    let val = &input.value;
    let mut prev = 0;
    for row in val.iter().rev() {
        let cur = row.get(row.len() - 1).unwrap();
        prev += cur;
    }

    prev
}

struct DerivationResult {
    value: Vec<Vec<i64>>,
}

fn derive_all(input: Vec<Vec<i64>>) -> Vec<DerivationResult> {
    input
        .iter()
        .map(|values| derive_until_zero(values.clone()))
        .collect()
}

fn derive_until_zero(input: Vec<i64>) -> DerivationResult {
    if all_zeroes(&input) {
        return DerivationResult { value: vec![input] };
    }
    let mut res = derive_until_zero(derive_once(&input));
    let mut value = vec![input];
    value.append(&mut res.value);
    DerivationResult { value }
}

fn all_zeroes(v: &Vec<i64>) -> bool {
    v.iter().all(|item| *item == 0)
}

fn derive_once(input: &Vec<i64>) -> Vec<i64> {
    let mut res = vec![];
    for i in 0..input.len() - 1 {
        let delta = input[i + 1] - input[i];
        res.push(delta);
    }
    res
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_input() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let ans = solution(input);
        assert_eq!(ans, 114)
    }

    #[test]
    fn test_derive() {
        let input = vec![0, 1, 2];
        let res = derive_once(&input);
        assert_eq!(res, vec![1, 1]);

        let input = vec![0, 0, 0];
        let res = derive_once(&input);
        assert_eq!(res, vec![0, 0]);

        let input = vec![3, 6, -12];
        let res = derive_once(&input);
        assert_eq!(res, vec![3, -18]);
    }

    #[test]
    fn test_derive_until_zero() {
        println!("Hello, world!");
        let input = vec![0, 1, 2];
        let res = derive_until_zero(input);
        assert_eq!(res.value, vec![vec![0, 1, 2], vec![1, 1], vec![0]]);
    }
}
