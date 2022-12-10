// Which is Greater?
// https://open.kattis.com/problems/whichisgreater

use std::io;

fn main() {
    let mut s: String = String::new();
    io::stdin().read_line(&mut s).expect("could not read");

    let v: Vec<&str> = s.split(" ").collect();
    let a: i32 = v[0].trim().parse().unwrap();
    let b: i32 = v[1].trim().parse().unwrap();

    if a <= b {
        println!("0");
    } else {
        println!("1")
    }
}
