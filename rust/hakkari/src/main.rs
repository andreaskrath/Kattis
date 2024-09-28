// Hakkari
// https://open.kattis.com/problems/hakkari

use std::io::stdin;

fn main() {
    let mut s_incoming_lines = String::new();
    stdin()
        .read_line(&mut s_incoming_lines)
        .expect("failed to read stdin");
    let incoming_lines: usize = s_incoming_lines
        .split_ascii_whitespace()
        .next()
        .expect("failed to get first element")
        .parse()
        .expect("failed to parse line amount");

    let mut lines = Vec::new();
    for _ in 0..incoming_lines {
        let mut temp = String::new();
        stdin().read_line(&mut temp).expect("failed to read stdin");
        lines.push(temp);
    }

    let bombs = solution(lines);
    println!("{}", bombs.len());
    for (x, y) in bombs {
        println!("{x} {y}",);
    }
}

fn solution(rows: Vec<String>) -> Vec<(usize, usize)> {
    let mut bombs = Vec::new();

    for (x, row) in rows.into_iter().enumerate() {
        for (y, cell) in row.char_indices() {
            if cell == '*' {
                bombs.push((x + 1, y + 1));
            }
        }
    }

    bombs
}

#[cfg(test)]
mod tests {
    use crate::solution;

    #[test]
    fn sample_1() {
        let input = vec![
            String::from("***"),
            String::from("..."),
            String::from("..."),
            String::from("..."),
        ];
        let expected = vec![(1, 1), (1, 2), (1, 3)];

        let actual = solution(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_2() {
        let input = vec![
            String::from("***"),
            String::from("*.*"),
            String::from("***"),
        ];
        let expected = vec![
            (1, 1),
            (1, 2),
            (1, 3),
            (2, 1),
            (2, 3),
            (3, 1),
            (3, 2),
            (3, 3),
        ];

        let actual = solution(input);

        assert_eq!(actual, expected);
    }
}
