pub fn elves_look_elves_say(mut str: String) -> usize {
    for _ in 0..40 {
        str = look_and_say(str);
    }

    str.len()
}

fn look_and_say(str: String) -> String {
    let mut chars = str.chars();

    if chars.clone().count() == 1 {
        return "1".to_owned() + &str;
    }

    let mut new_str = String::new();
    let mut last_char = chars.next().unwrap();
    let mut num_repeating: u32 = 1;

    for char in chars {
        if char == last_char {
            num_repeating += 1;
        } else {
            new_str += &(num_repeating.to_string() + &last_char.to_string());
            last_char = char;
            num_repeating = 1;
        }
    }

    if num_repeating > 0 {
        new_str += &(num_repeating.to_string() + &str.chars().last().unwrap().to_string());
    }

    new_str
}

#[cfg(test)]
mod part1 {
    #[test]
    fn example() {
        assert_eq!(super::look_and_say("1".to_string()), "11");
        assert_eq!(super::look_and_say("11".to_string()), "21");
        assert_eq!(super::look_and_say("21".to_string()), "1211");
        assert_eq!(super::look_and_say("1211".to_string()), "111221");
        assert_eq!(super::look_and_say("111221".to_string()), "312211");
    }

    #[test]
    fn input() {
        assert_eq!(super::elves_look_elves_say("1113222113".to_string()), 252594);
    }
}

pub fn elves_look_elves_say_part2(mut str: String) -> usize {
    for _ in 0..50 {
        str = look_and_say(str);
    }

    str.len()
}

#[cfg(test)]
mod part2 {
    #[test]
    fn input() {
        assert_eq!(super::elves_look_elves_say_part2("1113222113".to_string()), 3579328);
    }
}
