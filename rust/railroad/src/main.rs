// Railroad
// https://open.kattis.com/problems/railroad2

use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    println!("{}", solution(&input));
}

fn solution(s: &str) -> String {
    let (_, y_switches) = s.split_once(' ').unwrap();

    if y_switches.trim().parse::<usize>().unwrap() % 2 == 0 {
        String::from("possible")
    } else {
        String::from("impossible")
    }
}

#[cfg(test)]
mod samples {
    use crate::solution;

    #[test]
    fn sample_one() {
        let input = String::from("1 0\n");
        let expected = String::from("possible");
        let actual = solution(&input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_two() {
        let input = String::from("0 2\n");
        let expected = String::from("possible");
        let actual = solution(&input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_three() {
        let input = String::from("1 3\n");
        let expected = String::from("impossible");
        let actual = solution(&input);
        assert_eq!(actual, expected);
    }
}
