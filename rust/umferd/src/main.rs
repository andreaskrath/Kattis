// Umferð
// https://open.kattis.com/problems/umferd

use std::io::stdin;

fn main() {
    let mut cell_amount = String::new();
    stdin()
        .read_line(&mut cell_amount)
        .expect("should be able to read stdin");
    let mut lane_amount = String::new();
    stdin()
        .read_line(&mut lane_amount)
        .expect("should be able to read stdin");
    let lane_amount: usize = lane_amount
        .trim()
        .parse()
        .expect("should be able to parse lanes to usize");

    let mut lanes = Vec::with_capacity(lane_amount);
    for _ in 0..lane_amount {
        let mut temp_lane = String::new();
        stdin()
            .read_line(&mut temp_lane)
            .expect("should be able to read stdin");
        lanes.push(temp_lane);
    }

    let res = solution(cell_amount, lanes);
    println!("{res}");
}

fn solution(cell_amount: String, lanes: Vec<String>) -> f64 {
    let cell_amount: usize = cell_amount
        .trim()
        .parse()
        .expect("should be able to parse cell_amount to usize");

    let mut occupied = 0;
    for lane in lanes.iter() {
        for c in lane.trim().chars() {
            if c == '#' {
                occupied += 1;
            }
        }
    }

    1.0 - occupied as f64 / (cell_amount as f64 * lanes.len() as f64)
}

#[cfg(test)]
mod tests {
    use crate::solution;

    #[test]
    fn sample_one() {
        let cell_amount = String::from("2");
        let lanes = vec![String::from(".#")];
        let expected = 0.5;
        let actual = solution(cell_amount, lanes);
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_two() {
        let cell_amount = String::from("4");
        let lanes = vec![String::from("#.#."), String::from(".###")];
        let expected = 0.375;
        let actual = solution(cell_amount, lanes);
        assert_eq!(actual, expected);
    }
}
