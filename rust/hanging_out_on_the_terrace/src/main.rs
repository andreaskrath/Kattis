// Hanging Out on the Terrace
// https://open.kattis.com/problems/hangingout

use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("failed to read stdin");
    let mut split = input.split_ascii_whitespace();
    let (limit, events) = (
        split
            .next()
            .expect("failed to get first element from split")
            .trim()
            .parse::<usize>()
            .expect("failed to parse limit"),
        split
            .next()
            .expect("failed to get second element from split")
            .trim()
            .parse::<usize>()
            .expect("failed to parse events"),
    );

    let mut event_list = Vec::with_capacity(events);
    for _ in 0..events {
        let mut temp = String::new();
        stdin().read_line(&mut temp).expect("failed to read stdin");
        event_list.push(temp);
    }

    let res = solution(limit, event_list);
    println!("{res}");
}

fn solution(limit: usize, event_list: Vec<String>) -> usize {
    let mut current: usize = 0;
    let mut denied: usize = 0;

    for event in event_list {
        let mut split = event.split_ascii_whitespace();
        match split
            .next()
            .expect("failed to get first element from split")
            .trim()
        {
            "enter" => {
                let num = split
                    .next()
                    .expect("failed to get second element from split")
                    .trim()
                    .parse::<usize>()
                    .expect("failed to parse num in enter");

                if num + current <= limit {
                    current += num;
                } else {
                    denied += 1;
                }
            }
            "leave" => {
                let num = split
                    .next()
                    .expect("failed to get second element from split")
                    .trim()
                    .parse::<usize>()
                    .expect("failed to parse num in leave");
                current -= num;
            }
            _ => unreachable!("a string should only ever be 'enter' or 'leave'"),
        }
    }

    denied
}

#[cfg(test)]
mod tests {
    use crate::solution;

    #[test]
    fn sample_one() {
        let limit = 4;
        let event_list = vec![
            String::from("enter 3"),
            String::from("enter 2"),
            String::from("leave 1"),
            String::from("enter 1"),
            String::from("enter 2"),
        ];
        let expected = 2;
        let actual = solution(limit, event_list);
        assert_eq!(actual, expected);
    }
}
