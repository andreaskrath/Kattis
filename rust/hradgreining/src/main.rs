// Hraðgreining
// https://open.kattis.com/problems/hradgreining

use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("failed to read stdin");
    let res = solution(input);
    println!("{res}")
}

fn solution<'a>(s: String) -> &'a str {
    match s.trim().contains("COV") {
        true => "Veikur!",
        false => "Ekki veikur!",
    }
}

#[cfg(test)]
mod tests {
    use crate::solution;

    #[test]
    fn sample_one() {
        let input = String::from("COV");
        let expected = "Veikur!";
        let actual = solution(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_two() {
        let input = String::from("COOOV");
        let expected = "Ekki veikur!";
        let actual = solution(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_three() {
        let input = String::from("AACOVAA");
        let expected = "Veikur!";
        let actual = solution(input);
        assert_eq!(actual, expected);
    }
}
