use itertools::Itertools;

pub fn no_such_thing_as_too_much(str: &str, amount: u32) -> u32 {
    let containers: Vec<u32> = str.lines().map(|s| s.parse().expect("input contained non-u32")).collect();
    let mut num_combinations = 0;

    for i in 1..(containers.len() + 1) {
        let combinations = containers.iter().combinations(i);

        for c in combinations {
            let mut sum = 0;
            let mut clone = c.clone();

            while let Some(n) = clone.pop() {
                sum += n;
            }

            if sum == amount {
                num_combinations += 1;
            }
        }
    };

    num_combinations
}

#[cfg(test)]
mod part1 {
    #[test]
    fn example() {
        assert_eq!(super::no_such_thing_as_too_much("\
20
15
10
5
5", 25), 4);
    }

    #[test]
    fn input() {
        assert_eq!(super::no_such_thing_as_too_much(&sugar::input(file!()), 150), 1304);
    }
}

pub fn no_such_thing_as_too_much_part2(str: &str, amount: u32) -> u32 {
    let containers: Vec<u32> = str.lines().map(|s| s.parse().expect("input contained non-u32")).collect();
    let mut num_combinations = 0;
    let mut found = false;

    for i in 1..(containers.len() + 1) {
        if found { break; }

        let combinations = containers.iter().combinations(i);

        for c in combinations {
            let mut sum = 0;
            let mut clone = c.clone();

            while let Some(n) = clone.pop() {
                sum += n;
            }

            if sum == amount {
                found = true;
                num_combinations += 1;
            }
        }
    };

    num_combinations
}

#[cfg(test)]
mod part2 {
    #[test]
    fn example() {
        assert_eq!(super::no_such_thing_as_too_much_part2("\
20
15
10
5
5", 25), 3);
    }

    #[test]
    fn input() {
        assert_eq!(super::no_such_thing_as_too_much_part2(&sugar::input(file!()), 150), 18);
    }
}
