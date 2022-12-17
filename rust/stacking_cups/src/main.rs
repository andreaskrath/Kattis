// Stacking Cups
// https://open.kattis.com/problems/cups

use std::io::stdin;

fn main() {
    let mut input_lines: String = String::new();
    stdin().read_line(&mut input_lines).unwrap();

    let mut cups: Vec<String> = Vec::new();
    for _ in 0..input_lines.trim().parse().unwrap() {
        let mut cup: String = String::new();
        stdin().read_line(&mut cup).unwrap();
        cups.push(cup);
    }

    let sorted_cups = calc_cup_order(&cups);
    for cup in sorted_cups {
        println!("{}", cup.color);
    }
}

fn calc_cup_order(cups: &Vec<String>) -> Vec<Cup> {
    let mut output: Vec<Cup> = Vec::new();
    for cup in cups {
        let split_cup: Vec<&str> = cup.split_whitespace().collect();

        if let Ok(val) = split_cup[0].trim().parse::<f64>() {
            let new_cup = Cup::new(val / 2.0, split_cup[1].to_string());
            output.push(new_cup);
        } else {
            let new_cup = Cup::new(
                split_cup[1].trim().parse::<f64>().unwrap(),
                split_cup[0].to_string(),
            );
            output.push(new_cup);
        }
    }
    output.sort_by(|a, b| a.radius.partial_cmp(&b.radius).unwrap());
    output
}

struct Cup {
    radius: f64,
    color: String,
}

impl Cup {
    fn new(radius: f64, color: String) -> Self {
        Self { radius, color }
    }
}

#[cfg(test)]
mod tests {
    use crate::calc_cup_order;

    #[test]
    fn sample() {
        let input: Vec<String> = vec![
            String::from("red 10\n"),
            String::from("10 blue\n"),
            String::from("green 7\n"),
        ];
        let expected: Vec<String> = vec![
            String::from("blue"),
            String::from("green"),
            String::from("red"),
        ];
        let result = calc_cup_order(&input);

        for (index, cup) in result.iter().enumerate() {
            assert_eq!(cup.color, expected[index]);
        }
    }
}
