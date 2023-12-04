pub fn not_quite_lisp(str: &str) -> i32 {
    let mut sum = 0;

    for char in str.chars() {
        match char {
            '(' => sum += 1,
            ')' => sum -= 1,
            _ => {}
        }
    }

    sum
}

#[cfg(test)]
mod part1 {
    #[test]
    fn example() {
        assert_eq!(super::not_quite_lisp("(())"), 0);
        assert_eq!(super::not_quite_lisp("()()"), 0);
        assert_eq!(super::not_quite_lisp("((("), 3);
        assert_eq!(super::not_quite_lisp("(()(()("), 3);
        assert_eq!(super::not_quite_lisp("))((((("), 3);
        assert_eq!(super::not_quite_lisp("())"), -1);
        assert_eq!(super::not_quite_lisp("))("), -1);
        assert_eq!(super::not_quite_lisp(")))"), -3);
        assert_eq!(super::not_quite_lisp(")())())"), -3);
    }

    #[test]
    fn input() {
        assert_eq!(super::not_quite_lisp(&sugar::input(file!())), 280);
    }
}

pub fn not_quite_lisp_part2(str: &str) -> i32 {
    let mut sum = 0;
    let mut position = 0;

    for char in str.chars() {
        match char {
            '(' => sum += 1,
            ')' => sum -= 1,
            _ => {}
        }

        position += 1;
        if sum < 0 { break }
    }

    position
}

#[cfg(test)]
mod part2 {
    #[test]
    fn example() {
        assert_eq!(super::not_quite_lisp_part2(")"), 1);
        assert_eq!(super::not_quite_lisp_part2("()())"), 5);
    }

    #[test]
    fn input() {
        assert_eq!(super::not_quite_lisp_part2(&sugar::input(file!())), 1797);
    }
}
