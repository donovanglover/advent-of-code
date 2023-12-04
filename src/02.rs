pub fn cube_conundrum(str: &str) -> u32 {
    let mut sum = 0;

    for line in str.lines() {
        let mut is_valid = true;
        let mut prev_game = false;
        let mut game_number = 0;
        let mut num = 0;
        let mut colors = std::collections::HashMap::from([
            ("red", 0),
            ("green", 0),
            ("blue", 0),
        ]);

        for word in line.split(' ') {
            if prev_game {
                if let Ok(number) = crate::chomp(word).parse() {
                    game_number = number;
                }

                prev_game = false;
                continue;
            }

            if word == "Game" {
                prev_game = true;
                continue;
            }

            if let Ok(number) = word.parse::<u32>() {
                num = number;
                continue;
            }

            if word.contains(',') {
                colors.entry(crate::chomp(word)).and_modify(|i| { *i += num });
            } else if word.contains(';') {
                colors.entry(crate::chomp(word)).and_modify(|i| { *i += num });

                if !valid(&colors) {
                    is_valid = false;
                    break;
                } else {
                    colors = std::collections::HashMap::from([
                        ("red", 0),
                        ("green", 0),
                        ("blue", 0),
                    ]);
                }
            } else {
                colors.entry(word).and_modify(|i| { *i += num });

                if !valid(&colors) {
                    is_valid = false;
                    dbg!(line);
                    dbg!(&colors);
                    break;
                }
            }
        }

        if is_valid {
            sum += game_number
        }
    }

    sum
}

fn valid(colors: &std::collections::HashMap<&str, u32>) -> bool {
    let limits = std::collections::HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ]);

    for (color, num) in colors {
        if let Some(limit) = limits.get(color) {
            if num > limit {
                return false;
            }
        }
    }

    true
}

#[cfg(test)]
mod part1 {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(cube_conundrum("\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"), 8)
    }

    #[test]
    fn input() {
        assert_eq!(cube_conundrum(&crate::input(file!())), 2716);
    }
}

pub fn cube_conundrum_part2(str: &str) -> u32 {
    let mut sum = 0;

    for line in str.lines() {
        let mut prev_game = false;
        let mut num = 0;
        let mut colors = std::collections::HashMap::from([
            ("red", 0),
            ("green", 0),
            ("blue", 0),
        ]);

        for word in line.split(' ') {
            if prev_game {
                prev_game = false;
                continue;
            }

            if word == "Game" {
                prev_game = true;
                continue;
            }

            if let Ok(number) = word.parse::<u32>() {
                num = number;
                continue;
            }

            let color = match word.contains(',') || word.contains(';') {
                true => crate::chomp(word),
                false => word,
            };

            if let Some(current_num) = colors.get(color) {
                if num > *current_num {
                    colors.entry(color).and_modify(|i| { *i = num });
                }
            }
        }

        let mut power = 1;

        for (_, num) in colors {
            power *= num;
        }

        sum += power
    }

    sum
}

#[cfg(test)]
mod part2 {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(cube_conundrum_part2("\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"), 2286)
    }

    #[test]
    fn input() {
        assert_eq!(cube_conundrum_part2(&crate::input(file!())), 72227);
    }
}
