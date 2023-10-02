// Spritt
// https://open.kattis.com/problems/spritt

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let mut line_one = String::new();
    reader
        .read_line(&mut line_one)
        .expect("failed to read stdin");
    let mut split = line_one.trim().split_ascii_whitespace();
    let lines = split
        .next()
        .expect("no element in iterator")
        .parse()
        .expect("failed to parse lines");
    let bottles: usize = split
        .next()
        .expect("no element in iterator")
        .parse()
        .expect("failed to parse bottles");

    let mut buffer = String::new();
    let mut needed = 0;
    for _ in 0..lines {
        reader.read_line(&mut buffer).expect("failed to read stdin");
        needed += buffer
            .trim()
            .parse::<usize>()
            .expect("failed to parse buffer");
        buffer.clear();
    }

    use std::cmp::Ordering as O;
    let res = match bottles.cmp(&needed) {
        O::Less => "Neibb",
        O::Equal | O::Greater => "Jebb",
    };
    println!("{res}");
}
