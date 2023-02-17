// Prsteni
// https://open.kattis.com/problems/prsteni

use std::{collections::VecDeque, io::stdin};

fn main() {
    let mut discarded = String::new();
    stdin().read_line(&mut discarded).unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let result = solution(&input);
    for ratio in result {
        println!("{}", ratio);
    }
}

fn solution(s: &str) -> Vec<String> {
    let mut split: VecDeque<usize> = s
        .split_whitespace()
        .map(|s| s.trim().parse::<usize>().unwrap())
        .collect();

    let mut output: Vec<String> = Vec::new();
    let base = split.pop_front().unwrap();
    for num in split {
        let divisor = gcd(base, num);

        if divisor != 1 {
            let temp_base = base / divisor;
            let num = num / divisor;
            output.push(format!("{}/{}", temp_base, num));
            continue;
        }

        output.push(format!("{}/{}", base, num));
    }

    output
}

fn gcd(a: usize, b: usize) -> usize {
    let (mut small, mut large): (usize, usize);
    if a > b {
        large = a;
        small = b;
    } else {
        large = b;
        small = a;
    }

    while small > 0 {
        let remainder = large % small;
        large = small;
        small = remainder;
    }

    large
}

#[cfg(test)]
mod samples {
    use crate::solution;

    #[test]
    fn sample_one() {
        let input = String::from("8 4 2\n");
        let expected: Vec<String> = vec![String::from("2/1"), String::from("4/1")];
        let actual = solution(&input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_two() {
        let input = String::from("12 3 8 4\n");
        let expected: Vec<String> = vec![
            String::from("4/1"),
            String::from("3/2"),
            String::from("3/1"),
        ];
        let actual = solution(&input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_three() {
        let input = String::from("300 1 1 300\n");
        let expected: Vec<String> = vec![
            String::from("300/1"),
            String::from("300/1"),
            String::from("1/1"),
        ];
        let actual = solution(&input);
        assert_eq!(actual, expected);
    }
}
