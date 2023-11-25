// Flatbökuveisla
// https://open.kattis.com/problems/flatbokuveisla

use std::io::stdin;

fn main() {
    let mut slices = String::new();
    let mut residents = String::new();
    stdin()
        .read_line(&mut slices)
        .expect("should be able to read stdin");
    stdin()
        .read_line(&mut residents)
        .expect("should be able to read stdin");
    let res = solution(slices.as_str(), residents.as_str());
    println!("{res}");
}

fn solution(slices: &str, residents: &str) -> i32 {
    let mut slices: i32 = slices
        .trim()
        .parse()
        .expect("should be able to parse str into i32");
    let residents: i32 = residents
        .trim()
        .parse()
        .expect("should be able to parse str into i32");

    while slices - residents >= 0 {
        slices -= residents;
    }

    slices
}

#[cfg(test)]
mod tests {
    use crate::solution;

    #[test]
    fn sample_one() {
        let slices = String::from("8");
        let residents = String::from("3");
        let expected = 2;
        let actual = solution(slices.as_str(), residents.as_str());
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_two() {
        let slices = String::from("20");
        let residents = String::from("7");
        let expected = 6;
        let actual = solution(slices.as_str(), residents.as_str());
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_three() {
        let slices = String::from("999999");
        let residents = String::from("10");
        let expected = 9;
        let actual = solution(slices.as_str(), residents.as_str());
        assert_eq!(actual, expected);
    }
}
