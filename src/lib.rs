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
