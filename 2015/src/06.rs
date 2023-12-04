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

        for i in x1..(x2 + 1) {
            for j in y1..(y2 + 1) {
                lights[i][j] = toggle && !lights[i][j] || turn_on;
            }
        }
    }

    for i in 0..lights.len() {
        num_lights += lights[i].into_iter().filter(|b| *b).count();
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
        assert_eq!(super::probably_a_fire_hazard(&crate::input(file!())), 377891);
    }
}
