pub fn probably_a_fire_hazard(str: &str) -> usize {
    let mut num_lights = 0;
    let mut lights = [[false; 1000]; 1000];

    for line in str.lines() {
        let mut words = line.split(' ');
        let mut toggle = false;
        let mut turn_on = false;

        match words.next() {
            Some("toggle") => {
                toggle = true;
            },
            Some("turn") => {
                if let Some(maybe_on) = words.next() {
                    turn_on = maybe_on == "on";
                };
            },
            _ => {}
        }

        let a: Vec<&str> = match words.next() {
            Some(c1) => c1.split(',').collect(),
            None => vec![],
        };

        let b: Vec<&str> = match words.last() {
            Some(c2) => c2.split(',').collect(),
            None => vec![],
        };

        let x1: usize = a[0].parse().unwrap();
        let y1: usize = a[1].parse().unwrap();
        let x2: usize = b[0].parse().unwrap();
        let y2: usize = b[1].parse().unwrap();

        for row in lights.iter_mut().take(x2 + 1).skip(x1) {
            for light in row.iter_mut().take(y2 + 1).skip(y1) {
                *light = toggle && !*light || turn_on;
            }
        }
    }

    for light in &lights {
        num_lights += light.iter().filter(|b| **b).count();
    }

    num_lights
}

#[cfg(test)]
mod part1 {
    #[test]
    fn example() {
        assert_eq!(super::probably_a_fire_hazard("turn on 0,0 through 999,999"), 1000000);
        assert_eq!(super::probably_a_fire_hazard("toggle 0,0 through 999,0"), 1000);
        assert_eq!(super::probably_a_fire_hazard("turn off 499,499 through 500,500"), 0);
    }

    #[test]
    fn input() {
        assert_eq!(super::probably_a_fire_hazard(&sugar::input(file!())), 377891);
    }
}

pub fn probably_a_fire_hazard_part2(str: &str) -> usize {
    let mut num_lights = 0;
    let mut lights = vec![[0_usize; 1000]; 1000];

    for line in str.lines() {
        let mut words = line.split(' ');
        let mut toggle = false;
        let mut turn_on = false;

        match words.next() {
            Some("toggle") => {
                toggle = true;
            },
            Some("turn") => {
                if let Some(maybe_on) = words.next() {
                    turn_on = maybe_on == "on";
                };
            },
            _ => {}
        }

        let a: Vec<&str> = match words.next() {
            Some(c1) => c1.split(',').collect(),
            None => vec![],
        };

        let b: Vec<&str> = match words.last() {
            Some(c2) => c2.split(',').collect(),
            None => vec![],
        };

        let x1: usize = a[0].parse().unwrap();
        let y1: usize = a[1].parse().unwrap();
        let x2: usize = b[0].parse().unwrap();
        let y2: usize = b[1].parse().unwrap();

        for row in lights.iter_mut().take(x2 + 1).skip(x1) {
            for light in row.iter_mut().take(y2 + 1).skip(y1) {
                if toggle {
                    *light += 2;
                    continue;
                }

                if turn_on {
                    *light += 1;
                    continue;
                }

                if *light > 0 {
                    *light -= 1;
                }
            }
        }
    }

    for light in &lights {
        num_lights += light.iter().sum::<usize>();
    }

    num_lights
}

#[cfg(test)]
mod part2 {
    #[test]
    fn example() {
        assert_eq!(super::probably_a_fire_hazard_part2("turn on 0,0 through 0,0"), 1);
        assert_eq!(super::probably_a_fire_hazard_part2("toggle 0,0 through 999,999"), 2000000);
    }

    #[test]
    fn input() {
        assert_eq!(super::probably_a_fire_hazard_part2(&sugar::input(file!())), 14110788);
    }
}
