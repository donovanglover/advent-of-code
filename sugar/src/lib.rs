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

/// Given a string `line`, return a vector of all numbers on that line.
pub fn get_nums(line: &str) -> Vec<i32> {
    let words: Vec<&str> = line.split(' ').collect();
    let mut numbers: Vec<i32> = vec![];

    for maybe_number in words {
        if let Ok(number) = maybe_number.parse() {
            numbers.push(number);
        }
    }

    numbers

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chomp_test() {
        let result = chomp("03:");
        assert_eq!(result, "03");
    }
}
