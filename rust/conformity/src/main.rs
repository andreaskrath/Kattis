// Conformity
// https://open.kattis.com/problems/conformity

use std::{
    collections::HashMap,
    io::{stdin, BufRead},
};

fn main() {
    let stdin = stdin();
    let mut reader = stdin.lock();

    let mut cases = String::new();
    reader.read_line(&mut cases).expect("failed to read stdin");

    let mut selections = Vec::new();
    let mut temp = String::new();
    for _ in 0..cases.trim().parse().expect("failed to parse cases") {
        reader.read_line(&mut temp).expect("failed to read stdin");
        selections.push(temp.clone());
        temp.clear();
    }

    let res = solution(selections);
    println!("{res}");
}

fn solution(selections: Vec<String>) -> usize {
    let mut map: HashMap<String, usize> = HashMap::new();

    for selection in selections.iter() {
        let mut split: Vec<&str> = selection.split_ascii_whitespace().collect();
        split.sort_unstable();
        map.entry(split.join(" "))
            .and_modify(|n| *n += 1)
            .or_insert(1);
    }

    let res = *map.values().max().expect("map is empty");
    if res == 1 {
        selections.len()
    } else {
        let count = map.values().filter(|n| **n == res).count();
        if count == 1 {
            res
        } else {
            res * count
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::solution;

    #[test]
    fn sample_one() {
        let selections = vec![
            String::from("100 101 102 103 488"),
            String::from("100 200 300 101 102"),
            String::from("103 102 101 488 100"),
        ];
        let expected = 2;
        let actual = solution(selections);
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_two() {
        let selections = vec![
            String::from("200 202 204 206 208"),
            String::from("123 234 345 456 321"),
            String::from("100 200 300 400 444"),
        ];
        let expected = 3;
        let actual = solution(selections);
        assert_eq!(actual, expected);
    }
}
