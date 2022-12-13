// Free Food
// https://open.kattis.com/problems/freefood

use std::{collections::HashMap, io::stdin};

fn main() {
    let mut event_amount: String = String::new();
    stdin().read_line(&mut event_amount).unwrap();

    let mut input: Vec<String> = Vec::new();
    for _ in 0..event_amount.trim().parse().unwrap() {
        let mut temp: String = String::new();
        stdin().read_line(&mut temp).unwrap();
        input.push(temp);
    }

    println!("{}", days_with_food(input));
}

fn days_with_food(v: Vec<String>) -> usize {
    let mut food_days: HashMap<i32, bool> = HashMap::new();
    for event in v {
        let days: Vec<i32> = event
            .split_whitespace()
            .into_iter()
            .map(|s| s.trim().parse::<i32>().unwrap())
            .collect();

        for num in days[0]..=days[1] {
            food_days.insert(num, true);
        }
    }

    food_days.len()
}

#[cfg(test)]
mod tests {
    use crate::days_with_food;

    #[test]
    fn first_sample() {
        let input: Vec<String> = vec![
            String::from("10 14\n"),
            String::from("13 17\n"),
            String::from("25 26\n"),
        ];
        assert_eq!(10, days_with_food(input));
    }

    #[test]
    fn second_sample() {
        let input: Vec<String> = vec![String::from("1 365\n"), String::from("20 28\n")];
        assert_eq!(365, days_with_food(input));
    }
}
