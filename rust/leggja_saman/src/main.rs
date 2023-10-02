// Leggja saman
// https://open.kattis.com/problems/leggjasaman

use std::io::stdin;

fn main() {
    let (mut a, mut b) = (String::new(), String::new());
    stdin().read_line(&mut a).expect("failed to read stdin");
    stdin().read_line(&mut b).expect("failed to read stdin");
    let res = solution(a, b);
    println!("{res}");
}

fn solution(a: String, b: String) -> usize {
    a.trim().parse::<usize>().expect("failed to parse a")
        + b.trim().parse::<usize>().expect("failed to parse b")
}

#[cfg(test)]
mod tests {
    use crate::solution;

    #[test]
    fn sample_one() {
        let (a, b) = (String::from("4"), String::from("3"));
        let expected = 7;
        let actual = solution(a, b);
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_two() {
        let (a, b) = (String::from("11"), String::from("31"));
        let expected = 42;
        let actual = solution(a, b);
        assert_eq!(actual, expected);
    }
}
