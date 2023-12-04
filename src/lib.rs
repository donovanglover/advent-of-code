#[path="01.rs"]
pub mod day1;

#[path="02.rs"]
pub mod day2;

#[path="03.rs"]
pub mod day3;

/// Given a string `str`, return `str` without the last character.
pub fn chomp(str: &str) -> &str {
    let mut chars = str.chars();
    chars.next_back();
    chars.as_str()
}

/// Given a `file!()`, return its input.
pub fn input(file: &str) -> String {
    if let Some(file) = std::path::Path::new(file).file_stem() {
        if let Some(file) = file.to_str() {
            if let Ok(input) = std::fs::read_to_string(format!("./input/{file}.txt")) {
                return input;
            }
        }
    }

    String::new()
}
