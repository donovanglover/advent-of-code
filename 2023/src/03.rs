pub fn gear_ratios(str: &str) -> u32 {
    let mut sum = 0;
    let grid = make_grid(str);

    for row in 0..grid.len() {
        let mut is_adjacent = false;
        let mut num_str = String::new();

        for col in 0..grid[row].len() {
            if grid[row][col].is_ascii_digit() {
                num_str.push(grid[row][col]);

                is_adjacent = is_adjacent ||
                    (col > 0 && is_symbol(&grid[row][col - 1])) ||
                    (col + 1 < grid[row].len() && is_symbol(&grid[row][col + 1])) ||
                    (row > 0 && (
                        (col > 0 && is_symbol(&grid[row - 1][col - 1])) ||
                        is_symbol(&grid[row - 1][col]) ||
                        (col + 1 < grid[row].len() && is_symbol(&grid[row - 1][col + 1]))
                    )) ||
                    (row + 1 < grid.len() && (
                        (col > 0 && is_symbol(&grid[row + 1][col - 1])) ||
                        is_symbol(&grid[row + 1][col]) ||
                        (col + 1 < grid[row].len() && is_symbol(&grid[row + 1][col + 1]))
                    ));
            } else if !num_str.is_empty() {
                if is_adjacent {
                    if let Ok(num) = num_str.parse::<u32>() {
                        sum += num;
                    }

                    is_adjacent = false;
                }

                num_str.clear();
            }
        }
    }

    sum
}

fn make_grid(str: &str) -> Vec<Vec<char>> {
    let mut grid = vec![];

    for line in str.lines() {
        let mut vec = vec![];

        for char in line.chars() {
            vec.push(char)
        }

        vec.push('.');

        grid.push(vec)
    }

    grid
}

fn is_symbol(char: &char) -> bool {
    char != &'.' && !char.is_ascii_digit()
}

#[cfg(test)]
mod part1 {
    #[test]
    fn example() {
        assert_eq!(super::gear_ratios("\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."), 4361)
    }

    #[test]
    fn input() {
        assert_eq!(super::gear_ratios(&sugar::input(file!())), 535351);
    }
}

pub fn gear_ratios_part2(str: &str) -> u32 {
    let mut sum = 0;
    let grid = make_grid(str);

    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            let mut numbers = vec![];

            if grid[row][col] == '*' {
                if col > 0 && grid[row][col - 1].is_ascii_digit() {
                    numbers.push(get_number(&grid[row], col - 1))
                }

                if col + 1 < grid[row].len() && grid[row][col + 1].is_ascii_digit() {
                    numbers.push(get_number(&grid[row], col + 1))
                }

                if row > 0 {
                    if col > 0 && grid[row - 1][col - 1].is_ascii_digit() {
                        numbers.push(get_number(&grid[row - 1], col - 1))
                    }

                    if grid[row - 1][col].is_ascii_digit() {
                        numbers.push(get_number(&grid[row - 1], col))
                    }

                    if col + 1 < grid[row].len() && grid[row - 1][col + 1].is_ascii_digit() {
                        numbers.push(get_number(&grid[row - 1], col + 1))
                    }

                }

                if row + 1 < grid.len() {
                    if col > 0 && grid[row + 1][col - 1].is_ascii_digit() {
                        numbers.push(get_number(&grid[row + 1], col - 1))
                    }

                    if grid[row + 1][col].is_ascii_digit() {
                        numbers.push(get_number(&grid[row + 1], col))
                    }

                    if col + 1 < grid[row].len() && grid[row + 1][col + 1].is_ascii_digit() {
                        numbers.push(get_number(&grid[row + 1], col + 1))
                    }

                }
            }

            numbers.dedup();

            if numbers.len() == 2 {
                sum += numbers[0] * numbers[1];
            }
        }
    }

    sum
}

fn get_number(list: &Vec<char>, i: usize) -> u32 {
    let mut nums = list[i].to_string();

    let mut left = i - 1;

    while list[left].is_ascii_digit() {
        nums.insert(0, list[left]);

        if left > 0 {
            left -= 1;
        } else {
            break;
        }
    }

    let mut right = i + 1;

    while list[right].is_ascii_digit() {
        nums.push(list[right]);

        if right < list.len() {
            right += 1;
        } else {
            break;
        }
    }

    if let Ok(num) = nums.parse() {
        return num;
    }

    0
}

#[cfg(test)]
mod part2 {
    #[test]
    fn example() {
        assert_eq!(super::gear_ratios_part2("\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."), 467835);
    }

    #[test]
    fn input() {
        assert_eq!(super::gear_ratios_part2(&sugar::input(file!())), 87287096);
    }
}
