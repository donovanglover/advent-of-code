pub fn jsabacusframework_io(str: &str) -> i32 {
    let mut num_str = String::new();
    let mut numbers: Vec<i32> = vec![];

    for char in str.chars() {
        if char == '-' || char.is_ascii_digit() {
            num_str.push(char);
        } else {
            if let Ok(number) = num_str.parse::<i32>() {
                numbers.push(number);
                num_str.clear();
            }
        }
    }

    numbers.iter().sum()
}

#[cfg(test)]
mod part1 {
    #[test]
    fn example() {
        assert_eq!(super::jsabacusframework_io("[1,2,3]"), 6);
        assert_eq!(super::jsabacusframework_io(r#"{"a":2,"b":4}"#), 6);
        assert_eq!(super::jsabacusframework_io("[[[3]]]"), 3);
        assert_eq!(super::jsabacusframework_io(r#"{"a":{"b":4},"c":-1}"#), 3);
        assert_eq!(super::jsabacusframework_io(r#"{"a":[-1,1]}"#), 0);
        assert_eq!(super::jsabacusframework_io(r#"[-1,{"a":1}]"#), 0);
        assert_eq!(super::jsabacusframework_io("[]"), 0);
        assert_eq!(super::jsabacusframework_io("{}"), 0);
    }

    #[test]
    fn input() {
        assert_eq!(super::jsabacusframework_io(&sugar::input(file!())), 119433);
    }
}

pub fn jsabacusframework_io_part2(str: &str) -> i32 {
    let mut num_str = String::new();
    let mut numbers: Vec<i32> = vec![];
    let mut string = String::from(str);

    while let Some(index) = string.find("red") {
        let mut beginning = index - 1;
        let mut ending = index + 1;
        let mut found_array = false;

        while let Some(maybe_left_brace) = string.chars().nth(beginning) {
            if maybe_left_brace == '{' {
                break;
            }

            if maybe_left_brace == '[' || maybe_left_brace == ',' {
                string.replace_range(beginning..ending + "red".len(), "");
                found_array = true;
                break;
            }

            beginning -= 1;
        };

        if found_array {
            continue;
        }

        while let Some(maybe_right_brace) = string.chars().nth(ending) {
            if maybe_right_brace == '}' {
                break;
            }

            ending += 1;
        }

        string.replace_range(beginning..ending, "");
    }

    for char in string.chars() {
        if char == '-' || char.is_ascii_digit() {
            num_str.push(char);
        } else {
            if let Ok(number) = num_str.parse::<i32>() {
                numbers.push(number);
                num_str.clear();
            }
        }
    }

    numbers.iter().sum()
}

#[cfg(test)]
mod part2 {
    #[test]
    fn example() {
        assert_eq!(super::jsabacusframework_io_part2("[1,2,3]"), 6);
        assert_eq!(super::jsabacusframework_io_part2(r#"[1,{"c":"red","b":2},3]"#), 4);
        assert_eq!(super::jsabacusframework_io_part2(r#"{"d":"red","e":[1,2,3,4],"f":5}"#), 0);
        assert_eq!(super::jsabacusframework_io_part2(r#"[1,"red",5]"#), 6);
    }

    /// Too high
    #[test]
    fn input() {
        assert_eq!(super::jsabacusframework_io_part2(&sugar::input(file!())), 114426);
    }
}
