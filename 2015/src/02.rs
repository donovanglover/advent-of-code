pub fn i_was_told_there_would_be_no_math(str: &str) -> u32 {
    let mut sum = 0;

    for line in str.lines() {
        let mut lwh = vec![];

        for num in line.split('x') {
            if let Ok(num) = num.parse::<u32>() {
                lwh.push(num);
            }
        }

        let sides = vec![lwh[0] * lwh[1], lwh[1] * lwh[2], lwh[2] * lwh[0]];

        if let Some(smallest) = sides.iter().min() {
            sum += (2 * sides[0]) + (2 * sides[1]) + (2 * sides[2]) + smallest;
        }
    }


    sum
}

#[cfg(test)]
mod part1 {
    #[test]
    fn example() {
        assert_eq!(super::i_was_told_there_would_be_no_math("2x3x4"), 58);
        assert_eq!(super::i_was_told_there_would_be_no_math("1x1x10"), 43);
    }

    #[test]
    fn input() {
        assert_eq!(super::i_was_told_there_would_be_no_math(&crate::input(file!())), 1588178);
    }
}
