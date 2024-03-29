use glob::glob;
use solution::Solution;
use std::{
    fs::{self, OpenOptions},
    io::Write,
};

mod solution;

fn main() {
    let start_file_lines = fs::read_to_string("../README.md")
        .unwrap()
        .split('\n')
        .count();
    let mut rust_solutions = read_solutions("../rust/*/src/main.rs", "Rust");
    let mut go_solutions = read_solutions("../go/*/main.go", "Go");
    go_solutions.append(&mut rust_solutions);
    go_solutions.sort_unstable_by_key(|s| s.name.clone());
    build_table(go_solutions);
    let end_files_lines = fs::read_to_string("../README.md")
        .unwrap()
        .split('\n')
        .count();

    println!(
        "{} lines were added to the README.md.",
        end_files_lines - start_file_lines
    );
}

fn build_table(solutions: Vec<Solution>) {
    let table_header = "|Problem Name|Language|\n";
    let table_orientation = "|:-:|:-:|\n";

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("../README.md")
        .unwrap();

    _ = file.write(table_header.as_bytes()).unwrap();
    _ = file.write(table_orientation.as_bytes()).unwrap();
    for entry in solutions {
        _ = file.write(entry.to_table_string().as_bytes()).unwrap();
    }
}

fn read_solutions(file_pattern: &str, prog_lang: &str) -> Vec<Solution> {
    let mut solutions: Vec<Solution> = Vec::new();

    for path in glob(file_pattern).unwrap() {
        let path = path.unwrap().into_os_string().into_string().unwrap();
        let file = fs::read_to_string(&path).unwrap();
        let lines: Vec<&str> = file.split('\n').collect();

        if !lines[0].starts_with("// ") || !lines[1].starts_with("// https") {
            panic!("something is wrong with meta in {path}");
        }

        let name = lines[0].trim_start_matches("//").trim();
        let link = lines[1].trim_start_matches("//").trim();

        solutions.push(Solution::new(
            name.to_string(),
            link.to_string(),
            String::from(prog_lang),
            path.trim_start_matches("../").trim().to_string(),
        ));
    }

    solutions
}
