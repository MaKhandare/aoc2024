use aoc2024::utils;

fn main() {
    let input = utils::read_input(4);

    println!("Part 1: {}", solve_part1(&input));
    println!("Part 2: {}", solve_part2(&input));
}

// TODO: there has to be a more elegant way

fn solve_part1(input: &str) -> i32 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let rows = grid.len();
    let cols = grid[0].len();

    let target = ['X', 'M', 'A', 'S'];

    let mut count = 0;

    // left to right
    for row in 0..rows {
        for col in 0..=(cols - 4) {
            if grid[row][col..col + 4] == target {
                count += 1
            }
        }
    }

    // top to bottom
    for col in 0..cols {
        for row in 0..=(rows - 4) {
            if (0..4).all(|i| grid[row + i][col] == target[i]) {
                count += 1
            }
        }
    }

    // diag top left to bot right
    for row in 0..=(rows - 4) {
        for col in 0..=(cols - 4) {
            if (0..4).all(|i| grid[row + i][col + i] == target[i]) {
                count += 1
            }
        }
    }

    // diag top right to bot left
    for row in 0..=(rows - 4) {
        for col in 3..cols {
            if (0..4).all(|i| grid[row + i][col - i] == target[i]) {
                count += 1;
            }
        }
    }

    // right to left
    for row in 0..rows {
        for col in 3..cols {
            if grid[row][col - 3..=col].iter().rev().eq(target.iter()) {
                count += 1;
            }
        }
    }

    // bot to top
    for col in 0..cols {
        for row in 3..rows {
            if (0..4).all(|i| grid[row - i][col] == target[i]) {
                count += 1;
            }
        }
    }

    // diag bot right to top left
    for row in 3..rows {
        for col in 3..cols {
            if (0..4).all(|i| grid[row - i][col - i] == target[i]) {
                count += 1;
            }
        }
    }

    // diag bot left to top right
    for row in 3..rows {
        for col in 0..=(cols - 4) {
            if (0..4).all(|i| grid[row - i][col + i] == target[i]) {
                count += 1;
            }
        }
    }

    count
}

fn solve_part2(input: &str) -> i32 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let rows = grid.len();
    let cols = grid[0].len();

    let mut count = 0;

    for row in 1..(rows - 1) {
        for col in 1..(cols - 1) {
            if grid[row][col] == 'A' {
                let top_left = grid[row - 1][col - 1];
                let top_right = grid[row - 1][col + 1];
                let bot_right = grid[row + 1][col + 1];
                let bot_left = grid[row + 1][col - 1];

                if (top_left == 'M' && bot_right == 'S') || (top_left == 'S' && bot_right == 'M') {
                    if (top_right == 'M' && bot_left == 'S')
                        || (top_right == 'S' && bot_left == 'M')
                    {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}
#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    const SAMPLE_INPUT2: &str = r".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........
";

    #[test]
    fn part1() {
        let result = super::solve_part1(SAMPLE_INPUT);
        assert_eq!(result, 18)
    }

    #[test]
    fn part2() {
        let result = super::solve_part2(SAMPLE_INPUT2);
        assert_eq!(result, 9)
    }
}
