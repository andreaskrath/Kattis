// Framtíðar FIFA
// https://open.kattis.com/problems/fifa

use std::io::stdin;

const CURRENT_YEAR: usize = 2022;

fn main() {
    let (mut a, mut b) = (String::new(), String::new());
    stdin().read_line(&mut a).expect("failed to read stdin");
    stdin().read_line(&mut b).expect("failed to read stdin");
    let res = solution(a, b);
    println!("{res}");
}

fn solution(total_improvements: String, improvements_per_year: String) -> usize {
    let total_improvements = total_improvements
        .trim()
        .parse::<usize>()
        .expect("failed to parse total_improvements");
    let improvements_per_year = improvements_per_year
        .trim()
        .parse::<usize>()
        .expect("failed to parse improvements_per_year");

    CURRENT_YEAR + (total_improvements / improvements_per_year)
}

#[cfg(test)]
mod tests {
    use crate::solution;

    #[test]
    fn sample_one() {
        let (total, per_year) = (String::from("5"), String::from("5"));
        let expected = 2023;
        let actual = solution(total, per_year);
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_two() {
        let (total, per_year) = (String::from("21"), String::from("3"));
        let expected = 2029;
        let actual = solution(total, per_year);
        assert_eq!(actual, expected);
    }
}
