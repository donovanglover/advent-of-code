pub fn scratchcards(str: &str) -> u32 {
    let mut sum = 0;

    for line in str.lines() {
        let mut is_winning = true;
        let mut winning = vec![];
        let mut points = 0;

        for word in line.split(' ') {
            if word == "Game" || word.contains(':') { continue; }

            if word == "|" {
                is_winning = false;
                continue;
            }

            if let Ok(number) = word.parse::<u32>() {
                if is_winning {
                    winning.push(number);
                    continue;
                };

                for card in &winning {
                    if number == *card {
                        if points > 0 {
                            points *= 2;
                        } else {
                            points = 1;
                        }
                    }
                }
            }
        }

        sum += points;
    }

    sum
}

#[cfg(test)]
mod part1 {
    #[test]
    fn example() {
        assert_eq!(super::scratchcards("\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"), 13);
    }

    #[test]
    fn input() {
        assert_eq!(super::scratchcards(&sugar::input(file!())), 20667);
    }
}

pub fn scratchcards_part2(str: &str) -> u32 {
    let mut game_number = 0;
    let mut copies = vec![1; str.lines().count()];

    for line in str.lines() {
        let mut is_winning = true;
        let mut winning = vec![];
        let mut num_matches = 0;

        game_number += 1;

        for word in line.split(' ') {
            if word == "Game" || word.contains(':') { continue; }

            if word == "|" {
                is_winning = false;
                continue;
            }

            if let Ok(number) = word.parse::<u32>() {
                if is_winning {
                    winning.push(number);
                    continue;
                };

                for card in &winning {
                    if number == *card {
                        num_matches += 1;
                    }
                }
            }
        }

        let mut n = 0;

        while n < copies[game_number - 1] {
            for i in 0..num_matches {
                copies[game_number + i] += 1;
            }

            n += 1;
        }
    }

    copies.iter().sum()
}

#[cfg(test)]
mod part2 {
    #[test]
    fn example() {
        assert_eq!(super::scratchcards_part2("\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"), 30);
    }

    #[test]
    fn input() {
        assert_eq!(super::scratchcards_part2(&sugar::input(file!())), 5833065);
    }
}
