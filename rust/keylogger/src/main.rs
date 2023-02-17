// Keylogger
// https://open.kattis.com/problems/keylogger

use std::io::stdin;

fn main() {
    let mut key_pressed: String = String::new();
    stdin().read_line(&mut key_pressed).unwrap();

    let mut input: Vec<String> = Vec::new();
    for _ in 0..key_pressed.trim().parse().unwrap() {
        let mut temp: String = String::new();
        stdin().read_line(&mut temp).unwrap();
        input.push(temp);
    }

    println!("{}", calc_key_presses(&input));
}

fn calc_key_presses(input: &[String]) -> String {
    let mut output: String = String::new();
    let mut is_caps_lock: bool = false;
    let mut shift_pressed: bool = false;

    for key in input.iter().map(|s| s.trim()).collect::<Vec<&str>>() {
        let mut letter: char = match key.trim().as_bytes() {
            b"clank" => 'a',
            b"bong" => 'b',
            b"click" => 'c',
            b"tap" => 'd',
            b"poing" => 'e',
            b"clonk" => 'f',
            b"clack" => 'g',
            b"ping" => 'h',
            b"tip" => 'i',
            b"cloing" => 'j',
            b"tic" => 'k',
            b"cling" => 'l',
            b"bing" => 'm',
            b"pong" => 'n',
            b"clang" => 'o',
            b"pang" => 'p',
            b"clong" => 'q',
            b"tac" => 'r',
            b"boing" => 's',
            b"boink" => 't',
            b"cloink" => 'u',
            b"rattle" => 'v',
            b"clock" => 'w',
            b"toc" => 'x',
            b"clink" => 'y',
            b"tuc" => 'z',
            b"whack" => ' ',
            // Special key presses
            b"pop" => {
                output.pop();
                continue;
            }
            b"bump" => {
                if is_caps_lock {
                    is_caps_lock = false;
                    continue;
                }
                is_caps_lock = true;
                continue;
            }
            b"dink" => {
                shift_pressed = true;
                continue;
            }
            b"thumb" => {
                shift_pressed = false;
                continue;
            }
            _ => {
                println!("Unexpected input: {key}");
                panic!();
            }
        };

        if is_caps_lock && !shift_pressed || !is_caps_lock && shift_pressed {
            letter = letter.to_ascii_uppercase();
        }
        output.push(letter);
    }

    output
}

#[cfg(test)]
mod samples {
    use crate::calc_key_presses;

    #[test]
    fn sample_one() {
        let input: Vec<String> = vec![
            String::from("clank\n"),
            String::from("bong\n"),
            String::from("click\n"),
            String::from("tap\n"),
            String::from("poing\n"),
            String::from("clonk\n"),
            String::from("clack\n"),
            String::from("ping\n"),
            String::from("tip\n"),
            String::from("cloing\n"),
            String::from("tic\n"),
            String::from("cling\n"),
            String::from("bing\n"),
            String::from("pong\n"),
            String::from("clang\n"),
            String::from("pang\n"),
            String::from("clong\n"),
            String::from("tac\n"),
            String::from("boing\n"),
            String::from("boink\n"),
            String::from("cloink\n"),
            String::from("rattle\n"),
            String::from("clock\n"),
            String::from("toc\n"),
            String::from("clink\n"),
            String::from("tuc\n"),
        ];
        let expected: String = String::from("abcdefghijklmnopqrstuvwxyz");
        let actual = calc_key_presses(&input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_two() {
        let input: Vec<String> = vec![
            String::from("bump\n"),
            String::from("tip\n"),
            String::from("whack\n"),
            String::from("bump\n"),
            String::from("clock\n"),
            String::from("clank\n"),
            String::from("pong\n"),
            String::from("boink\n"),
            String::from("whack\n"),
            String::from("pang\n"),
            String::from("tip\n"),
            String::from("tuc\n"),
            String::from("tuc\n"),
            String::from("clank\n"),
        ];
        let expected: String = String::from("I want pizza");
        let actual = calc_key_presses(&input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_three() {
        let input: Vec<String> = vec![
            String::from("dink\n"),
            String::from("pong\n"),
            String::from("clang\n"),
            String::from("whack\n"),
            String::from("bump\n"),
            String::from("click\n"),
            String::from("thumb\n"),
            String::from("clank\n"),
            String::from("pang\n"),
            String::from("boing\n"),
        ];
        let expected: String = String::from("NO cAPS");
        let actual = calc_key_presses(&input);
        assert_eq!(actual, expected);
    }
}
