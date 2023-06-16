// Fading Wind
// https://open.kattis.com/problems/fadingwind

use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let distance_traveled = solution(input);
    println!("{distance_traveled}");
}

fn solution(input: String) -> u32 {
    let split_input: Vec<u32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let (mut height, constant, mut velocity, mut wind) = (
        split_input[0],
        split_input[1],
        split_input[2],
        split_input[3],
    );

    let mut distance_traveled = 0;
    while height > 0 {
        velocity += wind;
        velocity -= 1.max(f32::floor(velocity as f32 / 10.0) as u32);

        if velocity >= constant {
            height += 1;
        }

        if 0 < velocity && velocity < constant {
            height -= 1;
            if height == 0 {
                velocity = 0;
            }
        }

        if velocity == 0 {
            height = 0;
        }

        distance_traveled += velocity;

        wind = wind.saturating_sub(1);
    }

    distance_traveled
}

#[cfg(test)]
mod solution {
    use crate::solution;

    #[test]
    fn sample_one() {
        let input = String::from("1 1 1 1");
        let expected = 1;
        let actual = solution(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_two() {
        let input = String::from("2 2 2 2");
        let expected = 9;
        let actual = solution(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_three() {
        let input = String::from("1 2 3 4");
        let expected = 68;
        let actual = solution(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_four() {
        let input = String::from("314 159 265 358");
        let expected = 581062;
        let actual = solution(input);
        assert_eq!(actual, expected);
    }
}
