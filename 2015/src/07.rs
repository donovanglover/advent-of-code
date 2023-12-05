use std::collections::HashMap;

pub fn some_assembly_required(str: &str) -> HashMap<&str, u16> {
    let mut map = HashMap::new();

    for line in str.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();

        match words.len() {
            3 => {
                if let Ok(number) = words[0].parse::<u16>() {
                    map.insert(words[2], number);
                } else if let Some(number) = map.get(words[0]) {
                    map.insert(words[2], *number);
                } else {
                    panic!("`{}` is undefined (assignment)", words[0])
                }
            },

            4 => {
                if let Ok(number) = words[1].parse::<u16>() {
                    map.insert(words[3], !number);
                } else if let Some(number) = map.get(words[1]) {
                    map.insert(words[3], !*number);
                } else {
                    panic!("`{}` is undefined (not operator)", words[1])
                }
            },

            5 => {
                let num_left = match words[0].parse::<u16>() {
                    Ok(number) => Some(number),
                    _ => map.get(words[0]).copied(),
                };

                let num_right = match words[2].parse::<u16>() {
                    Ok(number) => Some(number),
                    _ => map.get(words[2]).copied(),
                };

                if let Some(num_left) = num_left {
                    if let Some(num_right) = num_right {
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
                    } else {
                        panic!("words[2] `{}` has not been defined yet", words[2])
                    }
                } else {
                    panic!("words[0] `{}` has not been defined yet", words[0])
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
    #[ignore]
    fn input() {
        assert_eq!(super::some_assembly_required(&sugar::input(file!())), HashMap::new());
    }
}
