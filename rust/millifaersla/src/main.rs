// Millifærsla
// https://open.kattis.com/problems/millifaersla

use std::io::stdin;

fn main() {
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

    stdin()
        .read_line(&mut a)
        .expect("should be able to read stdin");
    stdin()
        .read_line(&mut b)
        .expect("should be able to read stdin");
    stdin()
        .read_line(&mut c)
        .expect("should be able to read stdin");

    let res = solution(a.as_str(), b.as_str(), c.as_str());
    println!("{res}");
}

fn solution(a: &str, b: &str, c: &str) -> String {
    let a: i32 = a
        .trim()
        .parse()
        .expect("should be able to parse str to i32");
    let b: i32 = b
        .trim()
        .parse()
        .expect("should be able to parse str to i32");
    let c: i32 = c
        .trim()
        .parse()
        .expect("should be able to parse str to i32");

    let min = i32::MAX.min(a).min(b).min(c);

    match min {
        x if x == a => String::from("Monnei"),
        x if x == b => String::from("Fjee"),
        x if x == c => String::from("Dolladollabilljoll"),
        _ => unreachable!("can only be a, b, or c"),
    }
}

#[cfg(test)]
mod tests {
    use crate::solution;

    #[test]
    fn sample_one() {
        let (a, b, c) = (String::from("3"), String::from("9"), String::from("7"));
        let expected = String::from("Monnei");
        let actual = solution(a.as_str(), b.as_str(), c.as_str());
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_two() {
        let (a, b, c) = (String::from("323"), String::from("19"), String::from("999"));
        let expected = String::from("Fjee");
        let actual = solution(a.as_str(), b.as_str(), c.as_str());
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_three() {
        let (a, b, c) = (String::from("40"), String::from("30"), String::from("20"));
        let expected = String::from("Dolladollabilljoll");
        let actual = solution(a.as_str(), b.as_str(), c.as_str());
        assert_eq!(actual, expected);
    }
}
