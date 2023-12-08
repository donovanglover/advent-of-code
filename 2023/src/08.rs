use std::collections::HashMap;

pub fn haunted_wasteland(str: &str) -> u32 {
    let mut direction = vec![];
    let mut nodes: HashMap<&str, [&str; 2]> = HashMap::new();
    let mut num_steps = 1;

    for line in str.lines() {
        if line.contains('=') {
            let first = line.split(' ').next().unwrap();
            let left = line.split('(').last().unwrap().split(',').next().unwrap();
            let right = line.split(' ').last().unwrap().split(')').next().unwrap();

            nodes.insert(first, [left, right]);
        } else if line.contains('L') || line.contains('R') {
            direction = line.chars().collect();
        }
    }

    let mut d = 0;
    let mut c = "AAA";

    loop {
        let node = nodes.get(&c).unwrap();

        match direction[d] {
            'L' => {
                if node[0] == "ZZZ" {
                    break;
                } else {
                    c = node[0];
                }
            },
            'R' => {
                if node[1] == "ZZZ" {
                    break;
                } else {
                    c = node[1];
                }
            },
            _ => {}
        }

        num_steps += 1;
        d += 1;

        if d == direction.len() {
            d = 0;
        }
    }

    num_steps
}

#[cfg(test)]
mod part1 {
    #[test]
    fn example() {
        assert_eq!(super::haunted_wasteland("\
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"), 2);
        assert_eq!(super::haunted_wasteland("\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"), 6);
    }

    #[test]
    fn input() {
        assert_eq!(super::haunted_wasteland(&sugar::input(file!())), 13301);
    }
}
