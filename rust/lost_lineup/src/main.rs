// Lost Lineup
// https://open.kattis.com/problems/lostlineup

use std::{collections::BTreeMap, io::stdin};

fn main() {
    let mut dropped = String::new();
    stdin().read_line(&mut dropped).unwrap();
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let result = solution(&input);
    println!("{}", result);
}

fn solution(s: &str) -> String {
    let mut btree_map: BTreeMap<usize, usize> = BTreeMap::new();

    let split: Vec<usize> = s
        .split_whitespace()
        .map(|s| s.trim().parse().unwrap())
        .collect();

    // Swapping key and value to make later iteration pre-sorted
    // adding with 2 to make up for zero-index and first index always given
    for (i, &item) in split.iter().enumerate() {
        btree_map.insert(item, i + 2);
    }

    let mut output = String::from("1 ");
    for (_, v) in btree_map {
        output.push_str(format!("{} ", v).as_str());
    }

    output.trim().to_string()
}

#[cfg(test)]
mod samples {
    use crate::solution;

    #[test]
    fn sample_one() {
        let input = String::from("0\n");
        let expected = String::from("1 2");
        let actual = solution(&input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_two() {
        let input = String::from("1 2 0\n");
        let expected = String::from("1 4 2 3");
        let actual = solution(&input);
        assert_eq!(actual, expected);
    }
}
