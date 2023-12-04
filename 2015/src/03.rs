pub fn perfectly_spherical_houses_in_a_vacuum(str: &str) -> usize {
    let mut x = 0;
    let mut y = 0;
    let mut map = std::collections::HashMap::new();

    map.insert(format!("0,0"), 0);

    for char in str.chars() {
        match char {
            '<' => x -= 1,
            '>' => x += 1,
            'v' => y -= 1,
            '^' => y += 1,
            _ => {}
        };

        map.insert(format!("{x},{y}"), 0);
    }

    map.len()
}

#[cfg(test)]
mod part1 {
    #[test]
    fn example() {
        assert_eq!(super::perfectly_spherical_houses_in_a_vacuum(">"), 2);
        assert_eq!(super::perfectly_spherical_houses_in_a_vacuum("^>v<"), 4);
        assert_eq!(super::perfectly_spherical_houses_in_a_vacuum("^v^v^v^v^v"), 2);
    }

    #[test]
    fn input() {
        assert_eq!(super::perfectly_spherical_houses_in_a_vacuum(&crate::input(file!())), 2592);
    }
}
