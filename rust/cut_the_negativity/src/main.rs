// Cut the Negativity
// https://open.kattis.com/problems/cutthenegativity

use std::io::{stdin, BufRead};

fn main() {
    let stdin = stdin();
    let mut reader = stdin.lock();

    let mut lines = String::new();
    reader.read_line(&mut lines).expect("failed to read stdin");
    let lines: usize = lines.trim().parse().expect("failed to parse lines");

    let mut flight = String::new();
    let mut flights = Vec::with_capacity(lines);
    for _ in 0..lines {
        reader.read_line(&mut flight).expect("failed to read stdin");
        flights.push(flight.clone());
        flight.clear();
    }
    let res = solution(flights);
    println!("{}", res.len());
    for r in res {
        println!("{}", r);
    }
}

fn solution(flights: Vec<String>) -> Vec<String> {
    let mut new_flights = Vec::new();

    for (i, s) in flights.iter().enumerate() {
        for (j, sub_s) in s.split_ascii_whitespace().enumerate() {
            let num: i32 = sub_s.parse().expect("failed to parse sub_s");

            if num > 0 {
                new_flights.push(format!("{} {} {}", i + 1, j + 1, num))
            }
        }
    }

    new_flights
}

#[cfg(test)]
mod tests {
    use crate::solution;

    #[test]
    fn sample_one() {
        let input = vec![
            String::from("-1 1 -1 2"),
            String::from("9 -1 -1 -1"),
            String::from("-1 3 -1 4"),
            String::from("7 1 2 -1"),
        ];
        let expected = vec![
            String::from("1 2 1"),
            String::from("1 4 2"),
            String::from("2 1 9"),
            String::from("3 2 3"),
            String::from("3 4 4"),
            String::from("4 1 7"),
            String::from("4 2 1"),
            String::from("4 3 2"),
        ];
        let actual = solution(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_two() {
        let input = vec![
            String::from("-1 -1 -1"),
            String::from("15 -1 -1"),
            String::from("2 2 -1"),
        ];
        let expected = vec![
            String::from("2 1 15"),
            String::from("3 1 2"),
            String::from("3 2 2"),
        ];
        let actual = solution(input);
        assert_eq!(actual, expected);
    }
}
