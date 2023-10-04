// Mumble Rap
// https://open.kattis.com/problems/mumblerap

use std::io::stdin;

fn main() {
    let mut length = String::new();
    stdin()
        .read_line(&mut length)
        .expect("failed to read stdin");
    let length: usize = length.trim().parse().expect("failed to parse length");

    let mut input = String::with_capacity(length);
    stdin().read_line(&mut input).expect("failed to read stdin");
    let res = solution(input);
    println!("{res}");
}

fn solution(input: String) -> usize {
    let mut scanning_num = false;
    let (mut start, mut end, mut max) = (0, 0, 0);

    for (i, c) in input.trim().chars().enumerate() {
        match c.is_ascii_digit() {
            true => match scanning_num {
                true => {
                    end = i;
                }
                false => {
                    scanning_num = true;
                    start = i;
                    end = i;
                }
            },
            false => match scanning_num {
                true => {
                    scanning_num = false;
                    let num: usize = input[start..=end].parse().expect("failed to parse slice");
                    max = max.max(num);
                }
                false => {}
            },
        }
    }

    if scanning_num {
        let num: usize = input[start..=end].parse().expect("failed to parse slice");
        max = max.max(num);
    }

    max
}

#[cfg(test)]
mod tests {
    use crate::solution;

    #[test]
    fn sample_one() {
        let input = String::from("ihave40dollarsinmybankaccount");
        let expected = 40;
        let actual = solution(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_two() {
        let input = String::from("yesterdayihad1001,BUTnowihave9999");
        let expected = 9999;
        let actual = solution(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_three() {
        let input = String::from("13twenty209sixty123");
        let expected = 209;
        let actual = solution(input);
        assert_eq!(actual, expected);
    }
}
