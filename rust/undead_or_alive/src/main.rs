// Undead or Alive
// https://open.kattis.com/problems/undeadoralive

use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("failed to read line");
    println!("{}", solution(&input));
}

fn solution(s: &str) -> &str {
    let smiley = s.contains(":)");
    let frown = s.contains(":(");

    match (smiley, frown) {
        (true, true) => "double agent",
        (true, false) => "alive",
        (false, true) => "undead",
        (false, false) => "machine",
    }
}

#[cfg(test)]
mod solution {
    use crate::solution;

    #[test]
    fn sample_one() {
        let input = "Hello, how are you? :)";
        let expected = "alive";
        let actual = solution(input);
        assert_eq!(actual, expected);
    }
    #[test]
    fn sample_two() {
        let input = "Hey there! :( What's up? :(";
        let expected = "undead";
        let actual = solution(input);
        assert_eq!(actual, expected);
    }
    #[test]
    fn sample_three() {
        let input = "::(Braaaains... are very useful for programming contests:))";
        let expected = "double agent";
        let actual = solution(input);
        assert_eq!(actual, expected);
    }
    #[test]
    fn sample_four() {
        let input = "Sandy, when will my order be delivered?";
        let expected = "machine";
        let actual = solution(input);
        assert_eq!(actual, expected);
    }
    #[test]
    fn sample_five() {
        let input = "Firing up EmoticonBot... (:  : (  ):  :D  c:";
        let expected = "machine";
        let actual = solution(input);
        assert_eq!(actual, expected);
    }
}
