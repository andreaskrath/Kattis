// Odd Man Out
// https://open.kattis.com/problems/oddmanout

use std::{collections::HashMap, io::stdin};

fn main() {
    let mut cases = String::new();
    stdin().read_line(&mut cases).unwrap();

    let mut input: Vec<String> = Vec::new();
    for i in 0..cases.trim().parse::<usize>().unwrap() * 2 {
        let mut temp = String::new();
        stdin().read_line(&mut temp).unwrap();

        if i % 2 == 0 {
            continue;
        }

        input.push(temp);
    }

    let result = solution(&input);
    for (i, &val) in result.iter().enumerate() {
        println!("Case #{}: {}", i + 1, val);
    }
}

fn solution(v: &[String]) -> Vec<usize> {
    let mut output: Vec<usize> = Vec::new();
    for case in v {
        let mut map: HashMap<usize, usize> = HashMap::new();
        let split: Vec<usize> = case
            .split_whitespace()
            .map(|s| s.trim().parse().unwrap())
            .collect();

        // Collecting initial guests
        for num in split {
            match map.get(&num) {
                Some(val) => {
                    map.insert(num, val + 1);
                }
                None => {
                    map.insert(num, 1);
                }
            }
        }

        // Finding unique and adding to output vector
        for (k, v) in map {
            if v == 1 {
                output.push(k);
            }
        }
    }

    output
}

#[cfg(test)]
mod samples {
    use crate::solution;

    #[test]
    fn sample_one() {
        let input: Vec<String> = vec![
            String::from("1 2147483647 2147483647\n"),
            String::from("3 4 7 4 3\n"),
            String::from("2 10 2 10 5\n"),
        ];
        let expected: Vec<usize> = vec![1, 7, 5];
        let actual = solution(&input);
        assert_eq!(actual, expected);
    }
}
