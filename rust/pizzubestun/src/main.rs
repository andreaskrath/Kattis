// Pizzubestun
// https://open.kattis.com/problems/pizzubestun

use std::io::stdin;

fn main() {
    let mut amount = String::new();
    stdin().read_line(&mut amount).unwrap();
    let num: usize = amount.trim_end().parse().unwrap();

    let mut list = Vec::with_capacity(num);
    for _ in 0..num {
        let mut temp = String::new();
        stdin().read_line(&mut temp).unwrap();
        let s_number = temp.split_ascii_whitespace().nth(1).unwrap();
        list.push(s_number.parse::<u64>().unwrap());
    }

    list.sort_unstable();

    let sum: u64 = list
        .into_iter()
        .rev()
        .enumerate()
        .map(|(i, n)| if i % 2 == 0 { n } else { 0 })
        .sum();

    println!("{sum}");
}
