pub fn doesnt_he_have_intern_elves_for_this(str: &str) -> u32 {
    let mut num_nice = 0;

    for line in str.lines() {
        let mut num_vowels = 0;
        let mut has_repeat = false;
        let mut is_naughty = false;

        for (i, char) in line.char_indices() {
            if num_vowels < 3 {
                match char {
                    'a' | 'e' | 'i' | 'o' | 'u' => num_vowels += 1,
                    _ => {},
                };
            }

            if let Some(next) = line.chars().nth(i + 1) {
                has_repeat = has_repeat || (char == next);

                is_naughty = is_naughty ||
                    (char == 'a' && next == 'b') ||
                    (char == 'c' && next == 'd') ||
                    (char == 'p' && next == 'q') ||
                    (char == 'x' && next == 'y');
            };

            if is_naughty {
                break;
            }
        }

        if !is_naughty && has_repeat && num_vowels >= 3 {
            num_nice += 1;
        }
    }

    num_nice
}

#[cfg(test)]
mod part1 {
    #[test]
    fn example() {
        assert_eq!(super::doesnt_he_have_intern_elves_for_this("ugknbfddgicrmopn"), 1);
        assert_eq!(super::doesnt_he_have_intern_elves_for_this("aaa"), 1);
        assert_eq!(super::doesnt_he_have_intern_elves_for_this("jchzalrnumimnmhp"), 0);
        assert_eq!(super::doesnt_he_have_intern_elves_for_this("haegwjzuvuyypxyu"), 0);
        assert_eq!(super::doesnt_he_have_intern_elves_for_this("dvszwmarrgswjxmb"), 0);
    }

    #[test]
    fn input() {
        assert_eq!(super::doesnt_he_have_intern_elves_for_this(&crate::input(file!())), 258);
    }
}
