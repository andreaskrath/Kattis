// DRM Messages
// https://open.kattis.com/problems/drmmessages

use std::io::stdin;

const ASCII_OFFSET: u8 = 65;
const ASCII_AMOUNT: usize = 26;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("failed to read stdin");
    let res = solution(input);
    println!("{res}");
}

fn solution(input: String) -> String {
    // split
    let (s1, s2) = input.trim().split_at(input.len() / 2);

    // rotate
    let s1_sum: usize = s1.chars().map(|c| (c as u8 - ASCII_OFFSET) as usize).sum();
    let s1_rotated = s1
        .chars()
        .map(|c| {
            ((((c as u8 - ASCII_OFFSET) as usize + s1_sum) % ASCII_AMOUNT) as u8 + ASCII_OFFSET)
                as char
        })
        .collect::<String>();

    let s2_sum: usize = s2.chars().map(|c| (c as u8 - ASCII_OFFSET) as usize).sum();
    let s2_rotated = s2
        .chars()
        .map(|c| {
            ((((c as u8 - ASCII_OFFSET) as usize + s2_sum) % ASCII_AMOUNT) as u8 + ASCII_OFFSET)
                as char
        })
        .collect::<String>();

    // merge
    s1_rotated
        .chars()
        .zip(s2_rotated.chars())
        .map(|(c1, c2)| {
            ((((c1 as u8 + (c2 as u8 - ASCII_OFFSET)) - ASCII_OFFSET) as usize % ASCII_AMOUNT)
                as u8
                + ASCII_OFFSET) as char
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::solution;

    #[test]
    fn sample_one() {
        let input = String::from("EWPGAJRB");
        let expected = String::from("ABCD");
        let actual = solution(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_two() {
        let input = String::from("UEQBJPJCBUDGBNKCAHXCVERXUCVK");
        let expected = String::from("ACMECNACONTEST");
        let actual = solution(input);
        assert_eq!(actual, expected);
    }
}
