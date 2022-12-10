use std::{
    fs::{self, OpenOptions},
    io::Write,
};

use glob::glob;

fn main() {
    let rust_solutions = read_rust_solutions();
    build_table(rust_solutions);
}

// table builder
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

// read golang dir

fn read_rust_solutions() -> Vec<Solution> {
    let mut solutions: Vec<Solution> = Vec::new();

    for path in glob("../rust/*/src/main.rs").unwrap() {
        let path = path.unwrap().into_os_string().into_string().unwrap();
        let file = fs::read_to_string(&path).unwrap();
        let lines: Vec<&str> = file.split('\n').collect();

        let name = lines[0].trim_start_matches("//").trim();
        let link = lines[1].trim_start_matches("//").trim();

        solutions.push(Solution::new(
            name.to_string(),
            link.to_string(),
            String::from("Rust"),
            path.trim_start_matches("../").trim().to_string(),
        ));
    }

    solutions
}

#[derive(Debug)]
struct Solution {
    name: String,
    link: String,
    language: String,
    path: String,
}

impl Solution {
    fn new(name: String, link: String, language: String, path: String) -> Self {
        Self {
            name,
            link,
            language,
            path,
        }
    }

    fn to_table_string(&self) -> String {
        format!(
            "|[{}]({})|[{}]({})|\n",
            self.name, self.link, self.language, self.path
        )
    }
}
