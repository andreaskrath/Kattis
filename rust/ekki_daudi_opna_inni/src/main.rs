// Ekki dauði opna inni
// https://open.kattis.com/problems/ekkidaudi

use std::io::stdin;

fn main() {
    let mut line_one = String::new();
    let mut line_two = String::new();
    stdin()
        .read_line(&mut line_one)
        .expect("failed to read stdin");
    stdin()
        .read_line(&mut line_two)
        .expect("failed to read stdin");
    let res = solution(line_one.as_str(), line_two.as_str());
    println!("{res}");
}

fn solution(l1: &str, l2: &str) -> String {
    let mut s1 = l1.trim().split('|');
    let mut s2 = l2.trim().split('|');

    format!(
        "{}{} {}{}",
        s1.next().expect("failed to get first element in s1"),
        s2.next().expect("failed to get first element in s2"),
        s1.next().expect("failed to get second element in s1"),
        s2.next().expect("failed to get second element in s2"),
    )
}

#[cfg(test)]
mod tests {
    use crate::solution;

    #[test]
    fn sample_one() {
        let l1 = String::from("ho|lo");
        let l2 = String::from("pe|ve");
        let expected = String::from("hope love");
        let actual = solution(l1.as_str(), l2.as_str());
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_two() {
        let l1 = String::from("ekki |daudi");
        let l2 = String::from("opna| inni");
        let expected = String::from("ekki opna daudi inni");
        let actual = solution(l1.as_str(), l2.as_str());
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_three() {
        let l1 = String::from("you |dont");
        let l2 = String::from("matter| worry");
        let expected = String::from("you matter dont worry");
        let actual = solution(l1.as_str(), l2.as_str());
        assert_eq!(actual, expected);
    }
}
