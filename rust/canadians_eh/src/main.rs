// Canadians, eh?
// https://open.kattis.com/problems/canadianseh

use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("failed to read input");

    if input.trim().ends_with("eh?") {
        println!("Canadian!");
    } else {
        println!("Imposter!");
    }
}
