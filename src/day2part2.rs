pub fn cube_conundrum(str: &str) -> u32 {
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
                true => chomp(word),
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

fn chomp(str: &str) -> &str {
    let mut chars = str.chars();
    chars.next_back();
    chars.as_str()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(cube_conundrum("\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"), 2286)
    }

    #[test]
    fn input() {
        if let Ok(input) = std::fs::read_to_string("./input/02.txt") {
            assert_eq!(cube_conundrum(&input), 72227)
        }
    }
}
