pub fn the_ideal_stocking_stuffer(str: &str) -> u32 {
    for i in 1..u32::MAX {
        if format!("{:?}", md5::compute(str.to_owned() + &i.to_string())).starts_with("00000") {
            return i;
        }
    }

    0
}

#[cfg(test)]
mod part1 {
    #[test]
    fn example() {
        assert_eq!(super::the_ideal_stocking_stuffer("abcdef"), 609043);
        assert_eq!(super::the_ideal_stocking_stuffer("pqrstuv"), 1048970);
    }

    #[test]
    fn input() {
        assert_eq!(super::the_ideal_stocking_stuffer("iwrupvqb"), 346386);
    }
}

pub fn the_ideal_stocking_stuffer_part2(str: &str) -> u32 {
    for i in 1..u32::MAX {
        if format!("{:?}", md5::compute(str.to_owned() + &i.to_string())).starts_with("000000") {
            return i;
        }
    }

    0
}

#[cfg(test)]
mod part2 {
    #[test]
    fn input() {
        assert_eq!(super::the_ideal_stocking_stuffer_part2("iwrupvqb"), 9958218);
    }
}
