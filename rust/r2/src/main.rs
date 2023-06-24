// R2
// https://open.kattis.com/problems/r2

use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("failed to read stdin");
    let res = solution(input.trim());
    println!("{res}");
}

fn solution(input: &str) -> i16 {
    let split_input: Vec<&str> = input.split_whitespace().collect();
    let r1 = split_input
        .first()
        .expect("failed to get first")
        .parse::<i16>()
        .expect("failed to parse r1");
    let s = split_input
        .last()
        .expect("failed to get last")
        .parse::<i16>()
        .expect("failed to parse s");

    // median doubles with lowerbound subtracted gives the upper bound
    s * 2 - r1
}

#[cfg(test)]
mod tests {
    use crate::solution;

    #[test]
    fn sample_one() {
        let input = String::from("11 15");
        let actual = solution(&input);
        let expected = 19;
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_two() {
        let input = String::from("4 3");
        let actual = solution(&input);
        let expected = 2;
        assert_eq!(actual, expected);
    }
}
