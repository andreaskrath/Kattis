use std::io::stdin;

fn main() {
    let mut boy: String = String::new();
    let mut doctor: String = String::new();

    stdin()
        .read_line(&mut boy)
        .expect("failed to read boy input");
    stdin()
        .read_line(&mut doctor)
        .expect("failed to read doctor input");

    if boy.len() >= doctor.len() {
        println!("go");
    } else {
        println!("no");
    }
}
