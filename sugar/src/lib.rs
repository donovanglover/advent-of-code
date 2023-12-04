/// Given a string `str`, return `str` without the last character.
pub fn chomp(str: &str) -> &str {
    let mut chars = str.chars();
    chars.next_back();
    chars.as_str()
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
