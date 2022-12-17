// Birthday Memorization
// https://open.kattis.com/problems/fodelsedagsmemorisering

use std::{collections::HashMap, io::stdin};

fn main() {
    let mut test_cases: String = String::new();
    stdin().read_line(&mut test_cases).unwrap();

    let mut input: Vec<String> = Vec::new();

    for _ in 0..test_cases.trim().parse().unwrap() {
        let mut temp: String = String::new();
        stdin().read_line(&mut temp).unwrap();
        input.push(temp);
    }
    let result = make_birthday_list(&input);
    println!("{}", result.len());
    for name in result {
        println!("{name}");
    }
}

fn make_birthday_list(v: &Vec<String>) -> Vec<String> {
    let mut map: HashMap<&str, (i32, &str)> = HashMap::new();

    for person in v {
        let p_vec: Vec<&str> = person.split_whitespace().collect();
        let (name, score, date) = (
            p_vec[0].trim(),
            p_vec[1].trim().parse::<i32>().unwrap(),
            p_vec[2].trim(),
        );

        match map.get(date) {
            Some((val, _)) => {
                if score.gt(val) {
                    map.insert(date, (score, name));
                }
            }
            None => _ = map.insert(date, (score, name)),
        }
    }

    let mut output: Vec<String> = Vec::new();
    for (_, name) in map.values() {
        output.push((*name).to_string());
    }

    output.sort_unstable();
    output
}

#[cfg(test)]
mod tests {
    use crate::make_birthday_list;

    #[test]
    fn first_sample() {
        let input: Vec<String> = vec![
            String::from("Sanna 1 16/03\n"),
            String::from("Simon 2 16/03\n"),
            String::from("Saga 3 14/10\n"),
        ];
        let expected: Vec<String> = vec![String::from("Saga"), String::from("Simon")];
        assert_eq!(expected.len(), 2);
        assert_eq!(expected, make_birthday_list(&input));
    }

    #[test]
    fn second_sample() {
        let input: Vec<String> = vec![
            String::from("Oden 78 03/12\n"),
            String::from("Tor 132 14/05\n"),
            String::from("Freja 10000 14/05\n"),
            String::from("Loke 512 12/10\n"),
            String::from("Hel 14 04/05\n"),
            String::from("Fjorgynn 532 13/05\n"),
            String::from("Hildegun 500 13/05\n"),
            String::from("Vindsval 17 03/12\n"),
            String::from("Snotra 20 04/05\n"),
            String::from("Kvaser 420 03/12\n"),
        ];
        let expected: Vec<String> = vec![
            String::from("Fjorgynn"),
            String::from("Freja"),
            String::from("Kvaser"),
            String::from("Loke"),
            String::from("Snotra"),
        ];
        assert_eq!(expected.len(), 5);
        assert_eq!(expected, make_birthday_list(&input));
    }
}
