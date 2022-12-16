// Arrangement
// https://open.kattis.com/problems/upprodun

use std::io::stdin;

fn main() {
    let mut rooms: String = String::new();
    stdin().read_line(&mut rooms).unwrap();

    let mut teams: String = String::new();
    stdin().read_line(&mut teams).unwrap();

    let (rooms, teams): (usize, usize) =
        (rooms.trim().parse().unwrap(), teams.trim().parse().unwrap());
    let result = calc_team_division(rooms, teams);

    for room in result {
        println!("{}", "*".repeat(room));
    }
}

fn calc_team_division(rooms: usize, teams: usize) -> Vec<usize> {
    let mut division: Vec<usize> = vec![0; rooms];

    for index in 0..teams {
        let index = index % rooms;
        division[index] += 1;
    }

    division
}

// Test cases not applicable due to poor formatting from Kattis.
