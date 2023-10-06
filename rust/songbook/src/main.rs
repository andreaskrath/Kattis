// Songbook
// https://open.kattis.com/problems/sangbok

use std::io::stdin;

fn main() {
    let mut line_one = String::new();
    stdin()
        .read_line(&mut line_one)
        .expect("failed to read stdin");
    let mut line_two = String::new();
    stdin()
        .read_line(&mut line_two)
        .expect("failed to read stdin");
    let minutes: usize = line_one
        .split_ascii_whitespace()
        .next()
        .expect("failed to get minutes")
        .parse()
        .expect("failed to parse minutes");
    let res = solution(minutes, line_two);
    println!("{res}");
}

fn solution(minutes: usize, songs: String) -> usize {
    let seconds = minutes * 60;
    let mut current = 0;
    let mut songs: Vec<usize> = songs
        .split_ascii_whitespace()
        .map(|s| s.trim().parse().expect("failed to parse s"))
        .collect();
    songs.sort_unstable();

    for song in songs {
        match current + song < seconds {
            true => current += song,
            false => break,
        }
    }

    current
}

#[cfg(test)]
mod tests {
    use crate::solution;

    #[test]
    fn sample_one() {
        let (minutes, songs) = (4, String::from("110 110 110 110"));
        let expected = 220;
        let actual = solution(minutes, songs);
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_two() {
        let (minutes, songs) = (1, String::from("30 20"));
        let expected = 50;
        let actual = solution(minutes, songs);
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_three() {
        let (minutes, songs) = (2, String::from("60 120 250 299 1"));
        let expected = 61;
        let actual = solution(minutes, songs);
        assert_eq!(actual, expected);
    }
}
