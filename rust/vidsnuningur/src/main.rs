// Viðsnúningur
// https://open.kattis.com/problems/vidsnuningur

use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("failed to read stdin");
    let res = solution(input.as_str());
    println!("{res}");
}

fn solution(s: &str) -> String {
    s.trim().chars().rev().collect::<String>()
}

#[cfg(test)]
mod tests {
    use crate::solution;

    #[test]
    fn sample_one() {
        let input = String::from("2202annaloksdlahmarfinppekranutirroF");
        let expected = String::from("Forritunarkeppniframhaldskolanna2022");
        let actual = solution(input.as_str());
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_two() {
        let input = String::from("amma");
        let expected = String::from("amma");
        let actual = solution(input.as_str());
        assert_eq!(actual, expected);
    }
}
