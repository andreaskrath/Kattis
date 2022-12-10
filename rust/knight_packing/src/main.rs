// Knight Packing
// https://open.kattis.com/problems/knightpacking

use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("failed to read input");

    let num: i32 = input.trim().parse().expect("failed to parse");
    if num % 2 == 0 {
        println!("second");
    } else {
        println!("first");
    }
}
