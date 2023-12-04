use std::collections::HashMap;

pub fn aunt_sue(str: &str, aunts: &str) -> u32 {
    let aunts = get_aunts(aunts);
    let map = get_map(str);
    let mut n = 1;

    for aunt in aunts {
        let mut is_aunt = true;

        for (key, value) in aunt {
            if let Some(v) = map.get(key) {
                if *v != value {
                    is_aunt = false;
                    break;
                }
            } else {
                is_aunt = false;
                break;
            }
        }

        if is_aunt {
            return n
        } else {
            n += 1
        }
    }

    0
}

fn get_aunts(str: &str) -> Vec<HashMap<&str, u32>> {
    let mut aunts = vec![];

    for line in str.lines() {
        let mut map = HashMap::new();
        let mut current_word = "";

        for (i, word) in line.split(' ').enumerate() {
            if i < 2 { continue; }

            if word.contains(':') {
                current_word = word.split(':').next().unwrap();
            } else {
                map.insert(current_word, word.split(',').next().unwrap_or(word).parse().unwrap());
            }
        }

        aunts.push(map)
    }

    aunts
}

fn get_map(str: &str) -> HashMap<&str, u32> {
    let mut map = HashMap::new();

    for line in str.lines() {
        let mut words = line.split(' ');

        map.insert(sugar::chomp(words.next().unwrap()), words.next().unwrap().parse().unwrap());
    }

    map
}

#[cfg(test)]
mod part1 {
    #[test]
    fn input() {
        assert_eq!(super::aunt_sue("\
children: 3
cats: 7
samoyeds: 2
pomeranians: 3
akitas: 0
vizslas: 0
goldfish: 5
trees: 3
cars: 2
perfumes: 1", &sugar::input(file!())), 213);
    }
}

pub fn aunt_sue_part2(str: &str, aunts: &str) -> u32 {
    let aunts = get_aunts(aunts);
    let map = get_map(str);
    let mut n = 1;

    for aunt in aunts {
        let mut is_aunt = true;

        for (key, value) in aunt {
            if let Some(v) = map.get(key) {
                match key {
                    "cats" | "trees" => {
                        if value <= *v {
                            is_aunt = false;
                            break;
                        }
                    }

                    "pomeranians" | "goldfish" => {
                        if value >= *v {
                            is_aunt = false;
                            break;
                        }
                    }

                    _ => {
                        if *v != value {
                            is_aunt = false;
                            break;
                        }
                    }
                }
            } else {
                is_aunt = false;
                break;
            }
        }

        if is_aunt {
            return n
        } else {
            n += 1
        }
    }

    0
}

#[cfg(test)]
mod part2 {
    #[test]
    fn input() {
        assert_eq!(super::aunt_sue_part2("\
children: 3
cats: 7
samoyeds: 2
pomeranians: 3
akitas: 0
vizslas: 0
goldfish: 5
trees: 3
cars: 2
perfumes: 1", &sugar::input(file!())), 323);
    }
}
