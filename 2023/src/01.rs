pub fn trebuchet(str: &str) -> u32 {
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
mod part1 {
    #[test]
    fn example() {
        assert_eq!(super::trebuchet("
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
        "), 142)
    }

    #[test]
    fn input() {
        assert_eq!(super::trebuchet(&crate::input(file!())), 55621);
    }
}

pub fn trebuchet_part2(str: &str) -> u32 {
    let mut sum = 0;
    let str_nums = std::collections::HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);

    for line in str.lines() {
        let mut numbers = Vec::new();
        let mut string = String::new();

        for (i, char) in line.char_indices() {
            if char.is_ascii_digit() {
                numbers.push(char)
            } else {
                for (str_num, char_num) in &str_nums {
                    let j = i + str_num.len();

                    if let Some(s) = line.get(i..j) {
                        if s == *str_num {
                            numbers.push(*char_num)
                        }
                    }
                }
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
mod part2 {
    #[test]
    fn example() {
        assert_eq!(super::trebuchet_part2(";
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
        "), 281)
    }

    #[test]
    fn input() {
        assert_eq!(super::trebuchet_part2(&crate::input(file!())), 53592);
    }
}
