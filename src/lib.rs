pub mod day1part2;

pub fn day_one(str: &str) -> u32 {
    let mut sum = 0;

    for line in str.lines() {
        let mut numbers = Vec::new();
        let mut string = String::new();

        for char in line.chars() {
            if char.is_ascii_digit() {
                numbers.push(char)
            }
        }

        if let Some(first_digit) = numbers.first() {
            string.push(*first_digit)
        }

        if let Some(last_digit) = numbers.last() {
            string.push(*last_digit)
        }

        if let Ok(result) = string.parse::<u32>() {
            sum += result
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_one_example() {
        assert_eq!(day_one("
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
        "), 142)
    }

    #[test]
    fn day_one_input() {
        if let Ok(input) = std::fs::read_to_string("./input/01.txt") {
            assert_eq!(day_one(&input), 55621)
        }
    }
}
