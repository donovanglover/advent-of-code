#[path="01.rs"]
pub mod day1;

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
