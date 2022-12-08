use std::io::stdin;

fn main() {
    let (mut letter_string, mut number_string) = (String::new(), String::new());
    stdin()
        .read_line(&mut letter_string)
        .expect("failed to read letter_string");
    stdin()
        .read_line(&mut number_string)
        .expect("failed to read number_string");

    let mut output_string: String = String::new();
    let number_string: Vec<char> = number_string.trim().chars().collect();
    for id in number_string.chunks(3) {
        let str: String = id.iter().collect();
        let num: usize = str.trim().parse().unwrap();
        output_string.push(letter_string.chars().nth(num - 1).unwrap());
    }
    println!("{}", output_string);
}
