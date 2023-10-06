// The Bus Card
// https://open.kattis.com/problems/busskortet

use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("failed to read stdin");
    let res = solution(input);
    println!("{res}");
}

fn solution(input: String) -> u16 {
    let input: u16 = input.trim().parse().expect("failed to parse input");

    let bulk = input / 500;
    let current = bulk * 500;
    let mut transactions = bulk;

    match input - current {
        0 => {}
        100 | 200 => transactions += 1,
        300 | 400 => transactions += 2,
        remainder if remainder > 400 => transactions += 1,
        remainder if remainder > 300 => transactions += 2,
        remainder if remainder > 200 => transactions += 2,
        remainder if remainder > 100 => transactions += 1,
        _ => transactions += 1,
    }

    transactions
}

#[cfg(test)]
mod tests {
    use crate::solution;

    #[test]
    fn sample_one() {
        let input = String::from("850");
        let expected = 3;
        let actual = solution(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_two() {
        let input = String::from("1800");
        let expected = 5;
        let actual = solution(input);
        assert_eq!(actual, expected);
    }
}
