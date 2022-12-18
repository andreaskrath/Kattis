// Quick Estimates
// https://open.kattis.com/problems/quickestimate

use std::io::stdin;

fn main() {
    let mut cases_amount: String = String::new();
    stdin().read_line(&mut cases_amount).unwrap();

    let mut nums: Vec<String> = Vec::new();
    for _ in 0..cases_amount.trim().parse().unwrap() {
        let mut temp: String = String::new();
        stdin().read_line(&mut temp).unwrap();
        nums.push(temp);
    }

    let result = calc_len(nums);
    for len in result {
        println!("{len}");
    }
}

fn calc_len(nums: Vec<String>) -> Vec<usize> {
    let mut output: Vec<usize> = Vec::new();
    for num in nums {
        output.push(num.trim().len());
    }

    output
}

#[cfg(test)]
mod samples {
    use crate::calc_len;

    #[test]
    fn sample_one() {
        let input: Vec<String> = vec![
            String::from("314\n"),
            String::from("1\n"),
            String::from("5926\n"),
            String::from("5\n"),
            String::from("35897\n"),
        ];
        let expected: Vec<usize> = vec![3, 1, 4, 1, 5];
        let actual = calc_len(input);
        for (index, num) in expected.iter().enumerate() {
            assert_eq!(*num, actual[index], "index: {index}");
        }
    }

    #[test]
    fn sample_two() {
        let input: Vec<String> = vec![
            String::from("0\n"),
            String::from("10\n"),
            String::from("100\n"),
        ];
        let expected: Vec<usize> = vec![1, 2, 3];
        let actual = calc_len(input);
        for (index, num) in expected.iter().enumerate() {
            assert_eq!(*num, actual[index]);
        }
    }
}
