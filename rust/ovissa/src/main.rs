// Óvissa
// https://open.kattis.com/problems/ovissa

use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("failed to read from stdin");
    let res = solution(input);
    println!("{res}");
}

fn solution(s: String) -> usize {
    s.trim().len()
}

#[cfg(test)]
mod tests {
    use crate::solution;

    #[test]
    fn sample_one() {
        let input = String::from("uuuuu");
        let expected = 5;
        let actual = solution(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_two() {
        let input = String::from("uuuuuuuuuuuuuu");
        let expected = 14;
        let actual = solution(input);
        assert_eq!(actual, expected);
    }
}
