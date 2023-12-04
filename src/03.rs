pub fn gear_ratios(str: &str) -> u32 {
    let mut sum = 0;
    let grid = make_grid(str);

    for row in 0..grid.len() {
        let mut is_number = false;
        let mut is_adjacent = false;
        let mut num_str = String::new();

        // Note: This algorithm checks grid elements that have already been checked.
        for col in 0..grid[row].len() {
            if grid[row][col].is_ascii_digit() {
                is_number = true;

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

                continue;
            }

            if is_number {
                if is_adjacent {
                    if let Ok(num) = num_str.parse::<u32>() {
                        num_str.clear();
                        sum += num;
                    }
                } else {
                    num_str.clear();
                }
            }

            is_number = false;
            is_adjacent = false;
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

        grid.push(vec)
    }

    grid
}

fn is_symbol(char: &char) -> bool {
    char != &'.' && !char.is_ascii_digit()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use std::fs;

    #[test]
    fn example() {
        assert_eq!(gear_ratios("\
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
        let file = file!();

        if let Some(file) = Path::new(file).file_stem() {
            if let Some(file) = file.to_str() {
                if let Ok(input) = fs::read_to_string(format!("./input/{file}.txt")) {
                    assert_eq!(gear_ratios(&input), 99999999);
                }
            }
        }
    }
}
