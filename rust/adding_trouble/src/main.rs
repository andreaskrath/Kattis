// Adding Trouble
// https://open.kattis.com/problems/addingtrouble

use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    println!("{}", solution(&input));
}

fn solution(input: &str) -> &str {
    let split_input = input
        .split(' ')
        .map(|s| s.trim().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    if split_input[0] + split_input[1] == split_input[2] {
        "correct!"
    } else {
        "wrong!"
    }
}

#[cfg(test)]
mod solution {
    use crate::solution;

    #[test]
    fn sample_one() {
        let input = String::from("2 3 5");
        let expected = "correct!";
        let actual = solution(&input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_two() {
        let input = String::from("1 1 3");
        let expected = "wrong!";
        let actual = solution(&input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_three() {
        let input = String::from("-1 1 0");
        let expected = "correct!";
        let actual = solution(&input);
        assert_eq!(actual, expected);
    }
}
