// Stafur
// https://open.kattis.com/problems/stafur

use std::io::stdin;

/// "Y" is not considered a vowel for this problem
const VOWELS: &str = "AEIOU";
const CONSONANTS: &str = "BCDFGHJKLMNPQRSTVWXZ";
fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("failed to read stdin");
    let res = solution(input.as_str());
    println!("{res}");
}

fn solution(input: &str) -> &str {
    let input = input.trim();

    if VOWELS.contains(input) {
        "Jebb"
    } else if CONSONANTS.contains(input) {
        "Neibb"
    } else {
        "Kannski"
    }
}

#[cfg(test)]
mod tests {
    use crate::solution;

    #[test]
    fn sample_one() {
        let input = "A";
        let expected = "Jebb";
        let actual = solution(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_two() {
        let input = "B";
        let expected = "Neibb";
        let actual = solution(input);
        assert_eq!(actual, expected);
    }

    // not official sample, but ensures last path functions
    #[test]
    fn sample_three() {
        let input = "Y";
        let expected = "Kannski";
        let actual = solution(input);
        assert_eq!(actual, expected);
    }
}
