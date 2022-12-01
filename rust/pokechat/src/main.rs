use std::io::stdin;

fn main() {
    let (mut letter_string, mut id_string) = (String::new(), String::new());
    stdin()
        .read_line(&mut letter_string)
        .expect("failed to read letter_string");
    stdin()
        .read_line(&mut id_string)
        .expect("failed to read letter_string");

    // loop over id string and construct encoding string
    let mut output_string: String = String::new();

    for index in 0..id_string.chars().count() / 3 {
        let mut temp_str: String = String::new();
        for nested_index in 0..=2 {
            temp_str.push(
                id_string
                    .chars()
                    .nth((index * 3) + nested_index)
                    .expect("failed to push sub string to temp_str"),
            )
        }

        let current_id: usize = temp_str
            .parse()
            .expect("failed to parse temp_str int current_id");
        println!("current_id: {}", current_id);

        let temp_char: char = letter_string.chars().nth(current_id - 1).unwrap();

        output_string.push(temp_char);
    }

    // print output
    println!("{}", output_string.trim());
}
