use glob::glob;
use std::{
    fmt::Display,
    fs::{self, OpenOptions},
    io::Write,
};

fn main() {
    let start_file_lines = fs::read_to_string("../README.md").unwrap().lines().count();
    let mut rust_solutions = read_solutions("../rust/*/src/main.rs", Language::Rust);
    let mut go_solutions = read_solutions("../go/*/main.go", Language::Go);
    let mut haskell_solutions = read_solutions("../haskell/*/main.hs", Language::Haskell);
    go_solutions.append(&mut rust_solutions);
    go_solutions.append(&mut haskell_solutions);
    go_solutions.sort_unstable_by_key(|s| s.name.clone());
    build_table(go_solutions);
    let end_files_lines = fs::read_to_string("../README.md").unwrap().lines().count();

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
        .truncate(true)
        .open("../README.md")
        .expect("could not open README");

    _ = file.write(table_header.as_bytes()).unwrap();
    _ = file.write(table_orientation.as_bytes()).unwrap();
    for entry in solutions {
        _ = file.write(entry.to_table_string().as_bytes()).unwrap();
    }
}

fn read_solutions(file_pattern: &str, language: Language) -> Vec<Solution> {
    let mut solutions: Vec<Solution> = Vec::new();

    for path in glob(file_pattern).unwrap() {
        let path = path.unwrap().into_os_string().into_string().unwrap();
        let file = fs::read_to_string(&path).unwrap();
        let lines: Vec<&str> = file.lines().collect();

        if !lines[0].starts_with(language.comment())
            || !lines[1].starts_with(&format!("{} https", language.comment()))
        {
            panic!("something is wrong with meta in {path}");
        }

        let name = lines[0].trim_start_matches(language.comment()).trim();
        let link = lines[1].trim_start_matches(language.comment()).trim();

        solutions.push(Solution::new(
            name.to_string(),
            link.to_string(),
            language.to_string(),
            path.trim_start_matches("../").trim().to_string(),
        ));
    }

    solutions
}

struct Solution {
    pub name: String,
    link: String,
    language: String,
    path: String,
}

impl Solution {
    pub fn new(name: String, link: String, language: String, path: String) -> Self {
        Self {
            name,
            link,
            language,
            path,
        }
    }

    pub fn to_table_string(&self) -> String {
        format!(
            "|[{}]({})|[{}]({})|\n",
            self.name, self.link, self.language, self.path
        )
    }
}

enum Language {
    Rust,
    Go,
    Haskell,
}

impl Language {
    fn comment(&self) -> &str {
        match self {
            Language::Rust | Language::Go => "//",
            Language::Haskell => "--",
        }
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Language::Rust => "Rust",
            Language::Go => "Go",
            Language::Haskell => "Haskell",
        };

        write!(f, "{}", s)
    }
}
