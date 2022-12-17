// The Easiest Problem Is This One
// https://open.kattis.com/problems/easiest

use std::io::stdin;

fn main() {
    let mut input: Vec<usize> = Vec::new();

    loop {
        let mut temp: String = String::new();
        stdin().read_line(&mut temp).unwrap();

        let temp = temp.trim().parse::<usize>().unwrap();
        if temp == 0 {
            break;
        }

        input.push(temp);
    }
    let nums = calc_lowest_number(input);
    for num in nums {
        println!("{num}");
    }
}

fn calc_lowest_number(nums: Vec<usize>) -> Vec<usize> {
    let mut output: Vec<usize> = Vec::new();

    for num in nums {
        let target: usize = num
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .sum();

        // p must be bigger than 10
        let mut counter: usize = 11;
        'inner: loop {
            let current: usize = (num * counter)
                .to_string()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .sum();

            if current == target {
                output.push(counter);
                break 'inner;
            }

            counter += 1;
        }
    }

    output
}

#[cfg(test)]
mod samples {
    use crate::calc_lowest_number;

    #[test]
    fn sample() {
        let input: Vec<usize> = vec![3029, 4, 5, 42];
        let expected: Vec<usize> = vec![37, 28, 28, 25];
        let actual = calc_lowest_number(input);

        assert_eq!(actual, expected);
    }
}
