struct Race {
    time: i32,
    distance: i32,
}

pub fn wait_for_it(str: &str) -> u32 {
    let mut races: Vec<Race> = vec![];
    let mut nums = vec![];
    let mut product = 1;

    for line in str.lines() {
        nums.push(sugar::get_nums(line));
    }

    for i in 0..nums[0].len() {
        races.push(Race {
            time: nums[0][i],
            distance: nums[1][i],
        })
    }

    for race in races {
        let mut num_ways = 0;

        for distance_per_millisecond in 1..race.time {
            let time_to_move = race.time - distance_per_millisecond;

            if distance_per_millisecond * time_to_move > race.distance {
                num_ways += 1;
            }
        }

        product *= num_ways
    }

    product
}

#[cfg(test)]
mod part1 {
    #[test]
    fn example() {
        assert_eq!(super::wait_for_it("\
Time:      7  15   30
Distance:  9  40  200"), 288);
    }

    #[test]
    fn input() {
        assert_eq!(super::wait_for_it("\
Time:        49     87     78     95
Distance:   356   1378   1502   1882"), 503424);
    }
}

pub fn wait_for_it_part2(str: &str) -> u32 {
    let mut nums = vec![];

    for line in str.lines() {
        nums.push(sugar::get_nums(line));
    }

    let mut time = String::new();
    let mut distance = String::new();

    for i in 0..nums[0].len() {
        time += &nums[0][i].to_string();
        distance += &nums[1][i].to_string();
    }

    let time: i64 = time.parse().unwrap();
    let distance: i64 = distance.parse().unwrap();
    let mut num_ways = 0;

    for distance_per_millisecond in 1..time {
        let time_to_move = time - distance_per_millisecond;

        if distance_per_millisecond * time_to_move > distance {
            num_ways += 1;
        }
    }

    num_ways
}

#[cfg(test)]
mod part2 {
    #[test]
    fn example() {
        assert_eq!(super::wait_for_it_part2("\
Time:      7  15   30
Distance:  9  40  200"), 71503);
    }

    #[test]
    fn input() {
        assert_eq!(super::wait_for_it_part2("\
Time:        49     87     78     95
Distance:   356   1378   1502   1882"), 32607562);
    }
}
