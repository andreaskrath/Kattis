// Estimating the Area of a Circle
// https://open.kattis.com/problems/estimatingtheareaofacircle

use std::{f64::consts::PI, io::stdin};

fn main() {
    let mut circles: Vec<Circle> = Vec::new();
    loop {
        let mut temp: String = String::new();
        stdin().read_line(&mut temp).unwrap();
        let temp_vec: Vec<f64> = temp
            .split_whitespace()
            .map(|s| s.trim().parse::<f64>().unwrap())
            .collect();

        if temp_vec.iter().sum::<f64>() == 0.0 {
            break;
        }

        let new_circle: Circle = Circle::new(temp_vec[0], temp_vec[1], temp_vec[2]);
        circles.push(new_circle);
    }

    let result = calc_areas(&circles);
    for circle in result {
        println!("{} {}", circle.0, circle.1);
    }
}

fn calc_areas(circles: &Vec<Circle>) -> Vec<(f64, f64)> {
    // (actual, estimated)
    let mut output: Vec<(f64, f64)> = Vec::new();
    for circle in circles {
        let estimated: f64 =
            4.0 * circle.in_circle / circle.total_marks * circle.radius * circle.radius;
        let actual: f64 = PI * f64::powi(circle.radius, 2);
        output.push((actual, estimated));
    }

    output
}

struct Circle {
    radius: f64,
    total_marks: f64,
    in_circle: f64,
}

impl Circle {
    fn new(radius: f64, total_marks: f64, in_circle: f64) -> Self {
        Self {
            radius,
            total_marks,
            in_circle,
        }
    }
}

#[cfg(test)]
mod samples {
    use std::f64::consts::PI;

    use crate::{calc_areas, Circle};

    #[test]
    fn sample_one() {
        let input: Vec<Circle> = vec![
            Circle::new(1.0, 100.0, 75.0),
            Circle::new(10.0, 5000.0, 4023.0),
            Circle::new(3.0, 21.0, 17.0),
        ];
        let expected: Vec<(f64, f64)> = vec![
            (PI, 3.0),
            (314.159_265_4, 321.84),
            (28.274_333_88, 29.142_857_14),
        ];
        let result = calc_areas(&input);
        for (index, case) in result.iter().enumerate() {
            assert_eq!(case.0, expected[index].0);
            assert_eq!(case.1, expected[index].1);
        }
    }
}
