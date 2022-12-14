// Forced Choice
// https://open.kattis.com/problems/forcedchoice

use std::{collections::VecDeque, io::stdin};

fn main() {
    let mut first_line: String = String::new();
    stdin().read_line(&mut first_line).unwrap();
    let split_first_line: Vec<usize> = first_line
        .split_whitespace()
        .map(|s| s.trim().parse::<usize>().unwrap())
        .collect();
    // let cards_amount: usize = split_first_line[0];
    let prediction: usize = split_first_line[1];
    let steps: usize = split_first_line[2];

    let mut input: Vec<String> = Vec::new();
    for _ in 0..steps {
        let mut temp: String = String::new();
        stdin().read_line(&mut temp).unwrap();
        input.push(temp);
    }

    let result = decide_keep_or_remove(prediction, &input);
    for line in result {
        println!("{}", line);
    }
}

fn decide_keep_or_remove(p: usize, v: &Vec<String>) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();

    for line in v {
        let mut split_line: VecDeque<usize> = line
            .split_whitespace()
            .map(|s| s.trim().parse::<usize>().unwrap())
            .collect();

        split_line.pop_front();
        if split_line.contains(&p) {
            output.push(String::from("KEEP"));
        } else {
            output.push(String::from("REMOVE"));
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use crate::decide_keep_or_remove;

    #[test]
    fn sample() {
        let input = vec![
            String::from("2 1 5\n"),
            String::from("5 2 3 7 8 10\n"),
            String::from("3 2 7 10\n"),
            String::from("1 8\n"),
        ];
        let expected = vec![
            String::from("REMOVE"),
            String::from("KEEP"),
            String::from("REMOVE"),
            String::from("REMOVE"),
        ];
        assert_eq!(decide_keep_or_remove(3, &input), expected);
    }
}
