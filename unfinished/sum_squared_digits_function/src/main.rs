// Sum Squared Digits Function
// https://open.kattis.com/problems/sumsquareddigits

use std::io::stdin;

fn main() {
    let mut cases = String::new();
    stdin().read_line(&mut cases).unwrap();

    let mut input: Vec<String> = Vec::new();
    for _ in 0..cases.trim().parse().unwrap() {
        let mut temp = String::new();
        stdin().read_line(&mut temp).unwrap();
        input.push(temp);
    }

    let result = solution(input);
    for (i, &item) in result.iter().enumerate() {
        println!("{} {}", i + 1, item);
    }
}

fn solution(v: Vec<String>) -> Vec<u32> {
    let mut output: Vec<u32> = Vec::new();

    for case in v {
        let split: Vec<&str> = case.split_whitespace().collect();
        let base: u32 = split[1].trim().parse().unwrap();
        // let num = u32::from_str_radix(split[2].trim(), base)
        //     .unwrap()
        //     .to_string()
        //     .chars()
        //     .map(|c| c.to_digit(base).unwrap().pow(2))
        //     .sum();
        // let num: u32 = split[2]
        //     .trim()
        //     .chars()
        //     .map(|c|) // the closure should convert to correct base
        //     .sum();

        output.push(num);
    }

    output
}

#[cfg(test)]
mod samples {
    use crate::solution;

    #[test]
    fn sample_one() {
        let input = vec![
            String::from("1 10 1234\n"),
            String::from("2 3 98765\n"),
            String::from("3 16 987654321\n"),
        ];
        let expected: Vec<u32> = vec![30, 19, 696];
        let actual = solution(input);
        assert_eq!(actual, expected);
    }
}
