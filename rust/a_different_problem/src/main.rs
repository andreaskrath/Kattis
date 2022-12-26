// A Different Problem
// https://open.kattis.com/problems/different

use std::io::{stdin, BufRead};

fn main() {
    let mut input: Vec<String> = Vec::new();
    for line in stdin().lock().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }

        input.push(line);
    }

    let result = solution(input);
    for num in result {
        println!("{}", num);
    }
}

fn solution(v: Vec<String>) -> Vec<usize> {
    let mut output: Vec<usize> = Vec::new();

    for line in v {
        let nums: Vec<usize> = line
            .split_whitespace()
            .map(|s| s.trim().parse::<usize>().unwrap())
            .collect();
        let a = nums[0];
        let b = nums[1];

        let val = a.abs_diff(b);
        output.push(val);
    }

    output
}

#[cfg(test)]
mod samples {
    use crate::solution;

    #[test]
    fn sample_one() {
        let input: Vec<String> = vec![
            String::from("10 12\n"),
            String::from("71293781758123 72784\n"),
            String::from("1 12345677654321\n"),
        ];
        let expected: Vec<usize> = vec![2, 71_293_781_685_339, 12_345_677_654_320];
        let actual = solution(input);
        assert_eq!(actual, expected);
    }
}
