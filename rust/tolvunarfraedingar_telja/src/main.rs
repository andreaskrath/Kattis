// Tölvunarfræðingar telja
// https://open.kattis.com/problems/tolvunarfraedingartelja

use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("should be able to read stdin");
    let res = solution(input.as_str());
    println!("{res}");
}

fn solution(s: &str) -> usize {
    let num: usize = s
        .trim()
        .parse()
        .expect("should be able to parse str to usize");

    num - 1
}

#[cfg(test)]
mod tests {
    use crate::solution;

    #[test]
    fn sample_one() {
        let input = String::from("1");
        let expected = 0;
        let actual = solution(input.as_str());
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_two() {
        let input = String::from("2");
        let expected = 1;
        let actual = solution(input.as_str());
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_three() {
        let input = String::from("1337");
        let expected = 1336;
        let actual = solution(input.as_str());
        assert_eq!(actual, expected);
    }
}
