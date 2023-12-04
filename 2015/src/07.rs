use std::collections::HashMap;

pub fn some_assembly_required(str: &str) -> HashMap<&str, u16> {
    let mut map = HashMap::new();

    for line in str.lines() {
        let mut words: Vec<&str> = line.split_whitespace().collect();

        match words.len() {
            3 => {
                let last = *words.last().unwrap();
                let first = *words.first().unwrap();

                if let Ok(number) = first.parse::<u16>() {
                    map.insert(last, number);
                } else if let Some(b) = map.get(first) {
                    map.insert(last, *b);
                }
            },

            4 => {
                words.remove(0);

                let last = *words.last().unwrap();
                let first = *words.first().unwrap();

                if let Ok(number) = first.parse::<u16>() {
                    map.insert(last, !number);
                } else if let Some(first) = map.get(first) {
                    map.insert(last, !*first);
                }
            },

            5 => {
                let num_left = match words[0].parse::<u16>() {
                    Ok(number) => number,
                    _ => *map.get(words[0]).unwrap()
                };

                let num_right = match words[2].parse::<u16>() {
                    Ok(number) => number,
                    _ => *map.get(words[2]).unwrap()
                };

                match words[1] {
                    "AND" => {
                        map.insert(words[4], num_left & num_right);
                    },
                    "OR" => {
                        map.insert(words[4], num_left | num_right);
                    },
                    "LSHIFT" => {
                        map.insert(words[4], num_left << num_right);
                    },
                    "RSHIFT" => {
                        map.insert(words[4], num_left >> num_right);
                    },
                    _ => {}
                }
            }

            _ => {},
        }
    }

    map
}

#[cfg(test)]
mod part1 {
    use std::collections::HashMap;

    #[test]
    fn example() {
        assert_eq!(super::some_assembly_required("\
123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i"), HashMap::from([
("d", 72),
("e", 507),
("f", 492),
("g", 114),
("h", 65412),
("i", 65079),
("x", 123),
("y", 456),
]));
    }

    #[test]
    fn input() {
        assert_eq!(*super::some_assembly_required(&crate::input(file!())).get("a").unwrap(), 9999u16);
    }
}
