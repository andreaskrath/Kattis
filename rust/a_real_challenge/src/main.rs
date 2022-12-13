// A Real Challenge
// https://open.kattis.com/problems/areal

use std::io::stdin;

fn main() {
    let mut input: String = String::new();
    stdin().read_line(&mut input).unwrap();

    println!("{}", calc_fence_len(&input));
}

fn calc_fence_len(s: &str) -> f64 {
    s.trim().parse::<f64>().unwrap().sqrt() * 4.0
}

#[cfg(test)]
mod tests {
    use crate::calc_fence_len;

    #[test]
    fn first_sample() {
        let input = String::from("16");
        assert_eq!(calc_fence_len(&input), 16.0);
    }

    #[test]
    fn second_sample() {
        let input = "5";
        assert_eq!(calc_fence_len(input), 8.94427190999915878564);
    }
}
