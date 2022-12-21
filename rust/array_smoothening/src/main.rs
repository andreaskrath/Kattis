// Array Smoothening
// https://open.kattis.com/problems/arraysmoothening

use std::{
    collections::{BinaryHeap, HashMap},
    io::stdin,
};

fn main() {
    let mut temp = String::new();
    stdin().read_line(&mut temp).unwrap();
    let (_, remove_amount) = temp.split_once(' ').unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let result = solution(&input, remove_amount.trim().parse::<usize>().unwrap());
    println!("{result}");
}

fn solution(input: &str, mut remove_amount: usize) -> usize {
    let mut map: HashMap<usize, usize> = HashMap::new();
    let input_vec = input
        .split_whitespace()
        .map(|s| s.trim().parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    for num in input_vec {
        if let Some(val) = map.get_mut(&num) {
            *val += 1;
        } else {
            map.insert(num, 1);
        }
    }

    let mut occurances: BinaryHeap<usize> = map.values().into_iter().copied().collect();
    while remove_amount != 0 {
        let mut num = occurances.pop().unwrap();
        num -= 1;
        remove_amount -= 1;
        occurances.push(num);
    }

    occurances.pop().unwrap()
}

#[cfg(test)]
mod samples {
    use crate::solution;

    #[test]
    fn sample_one() {
        let input: String = String::from("1 1 3 3 3 3 3 7 7 7\n");
        let input_remove_amount: usize = 0;
        let expected: usize = 5;
        let actual = solution(&input, input_remove_amount);
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_two() {
        let input: String = String::from("3 1 7 3 3 3 7 3 1 7\n");
        let input_remove_amount: usize = 3;
        let expected: usize = 3;
        let actual = solution(&input, input_remove_amount);
        assert_eq!(actual, expected);
    }
}
