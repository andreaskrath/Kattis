// Class Field Trip
// https://open.kattis.com/problems/classfieldtrip

use std::io::stdin;
use std::iter::FromIterator;

fn main() {
    let mut s1 = String::new();
    let mut s2 = String::new();
    stdin().read_line(&mut s1).unwrap();
    stdin().read_line(&mut s2).unwrap();

    let mut s = String::from(s1.trim());
    s.push_str(s2.trim());

    let mut chars: Vec<char> = s.chars().collect();
    chars.sort_unstable();
    println!("{}", String::from_iter(chars));
}
