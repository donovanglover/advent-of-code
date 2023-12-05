use std::collections::HashMap;

pub fn if_you_give_a_seed_a_fertilizer(str: &str) -> u32 {
    let mut sum = 0;
    let mut seeds: Vec<i32> = vec![];
    let mut current_map = "";
    let mut map: HashMap<&str, [i32; 2]> = HashMap::new();

    for line in str.lines() {
        if line.starts_with("seeds:") {
            seeds = sugar::get_nums(line);
        } else if let Some(map) = line.strip_suffix(" map:") {
            current_map = map;
        } else {
            let nums = sugar::get_nums(line);

            if nums.len() == 3 {
                for seed in &seeds {
                    if seed >= &nums[1] && seed < &(nums[1] + nums[2]) {
                        dbg!(&nums, seed);
                        map.insert(current_map, [*seed, seed + (nums[0] - nums[1])]);
                        dbg!(seed, &map);
                    } else {
                        map.insert(current_map, [*seed, *seed]);
                    }
                }
            }
        }

        sum += 1;
    }

    sum
}

#[cfg(test)]
mod part1 {
    #[test]
    #[ignore]
    fn example() {
        assert_eq!(super::if_you_give_a_seed_a_fertilizer("\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"), 35);
    }

    #[test]
    #[ignore]
    fn input() {
        assert_eq!(super::if_you_give_a_seed_a_fertilizer(&sugar::input(file!())), 999999);
    }
}
