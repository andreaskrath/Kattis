// Above Average
// https://open.kattis.com/problems/aboveaverage

use std::{collections::VecDeque, io::stdin};

fn main() {
    let mut input_amount: String = String::new();
    stdin().read_line(&mut input_amount).unwrap();

    for _ in 0..input_amount.trim().parse::<usize>().unwrap() {
        let mut line: String = String::new();
        stdin().read_line(&mut line).unwrap();

        let mut line_vec: VecDeque<f64> = line
            .split_whitespace()
            .into_iter()
            .map(|n| n.trim().parse::<f64>().unwrap())
            .collect();

        let student_amount = line_vec.pop_front().unwrap();
        let average_grade: f64 = line_vec.iter().sum::<f64>() / student_amount;

        let mut above_average: f64 = 0.0;
        for grade in line_vec {
            if grade.gt(&average_grade) {
                above_average += 1.0;
            }
        }
        println!("{:.3}%", above_average / student_amount * 100.0);
    }
}
