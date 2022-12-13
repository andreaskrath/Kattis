// Coffee Cup Combo
// https://open.kattis.com/problems/coffeecupcombo

use std::io::stdin;

fn main() {
    let mut str_len: String = String::new();
    let mut input: String = String::new();
    stdin().read_line(&mut str_len);
    stdin().read_line(&mut input).unwrap();

    println!("{}", calc_awake_lectures(&input));
}

fn calc_awake_lectures(s: &str) -> usize {
    let mut awake_lectures: usize = 0;
    let mut reserve_coffee_cups = 0;

    for c in s.trim().chars() {
        match c {
            '1' => {
                awake_lectures += 1;
                reserve_coffee_cups = 2;
            }
            '0' => {
                if reserve_coffee_cups == 0 {
                    continue;
                }

                awake_lectures += 1;
                reserve_coffee_cups -= 1;
            }
            _ => unreachable!(),
        }
    }

    awake_lectures
}

#[cfg(test)]
mod tests {
    use crate::calc_awake_lectures;

    #[test]
    fn first_sample() {
        let input: String = String::from("0100010100");
        assert_eq!(calc_awake_lectures(&input), 8);
    }

    #[test]
    fn second_sample() {
        let input: String = String::from("1100000000");
        assert_eq!(calc_awake_lectures(&input), 4);
    }

    #[test]
    fn third_sample() {
        let input: String = String::from("0");
        assert_eq!(calc_awake_lectures(&input), 0);
    }
}
