// Amerískur vinnustaður
// https://open.kattis.com/problems/ameriskur

use std::io::stdin;

const FIELD_IN_KM: f64 = 0.09144;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("failed to read stdin");
    let res = solution(input.as_str());
    println!("{res}");
}

fn solution(len: &str) -> f64 {
    FIELD_IN_KM * len.trim().parse::<f64>().expect("failed to parse")
}

#[cfg(test)]
mod tests {
    use crate::solution;

    #[test]
    fn sample_one() {
        let input = String::from("1");
        let expected = 0.09144;
        let actual = solution(input.as_str());
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_two() {
        let input = String::from("3");
        let expected = 0.27432;
        let actual = solution(input.as_str());
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_three() {
        let input = String::from("1337");
        let expected = 122.25527999999998;
        let actual = solution(input.as_str());
        assert_eq!(actual, expected);
    }
}
