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
                    assert_eq!(gear_ratios(&input), 535351);
                }
            }
        }
    }
}
