// Reduplication
// https://open.kattis.com/problems/reduplikation

use std::io::stdin;

fn main() {
    let mut input_str = String::new();
    stdin()
        .read_line(&mut input_str)
        .expect("failed to read stdin");
    let mut input_num = String::new();
    stdin()
        .read_line(&mut input_num)
        .expect("failed to read stdin");

    let repeat_num = input_num.trim().parse().expect("failed to parse input num");
    println!("{}", input_str.trim().repeat(repeat_num));
}
