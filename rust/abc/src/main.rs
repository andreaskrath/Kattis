// ABC
// https://open.kattis.com/problems/abc

use std::io::stdin;

fn main() {
    let mut numbers: String = String::new();
    let mut order: String = String::new();

    stdin()
        .read_line(&mut numbers)
        .expect("failed to read number input");
    stdin()
        .read_line(&mut order)
        .expect("failed to read desired order");

    let mut numbers: Vec<usize> = numbers
        .split_whitespace()
        .into_iter()
        .map(|s| s.parse::<usize>().expect("failed to parse to int"))
        .collect();
    numbers.sort_unstable();
    let (a, b, c) = (numbers[0], numbers[1], numbers[2]);

    let mut output: String = String::new();
    for letter in order.trim().chars() {
        let temp = match letter {
            'A' => a,
            'B' => b,
            'C' => c,
            _ => unreachable!(),
        };
        output.push_str(format!("{} ", temp).as_str());
    }

    println!("{}", output.trim());
}
