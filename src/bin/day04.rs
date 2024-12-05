use aoc2024::utils;

fn main() {
    let input = utils::read_input(4);

    println!("Part 1: {}", solve_part1(&input));
    println!("Part 2: {}", solve_part2(&input));
}

fn solve_part1(input: &str) -> i32 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;

    let mut count = 0;
    let target = ['X', 'M', 'A', 'S'];

    // yx
    let directions = [
        (0, 1),   // left to right
        (0, -1),  // right to left
        (1, 0),   // top to bot
        (-1, 0),  // bot to top
        (1, 1),   // topleft to botright
        (1, -1),  // topright to botleft
        (-1, -1), // botright to topleft
        (-1, 1),  // botleft to topright
    ];

    for row in 0..rows {
        for col in 0..cols {
            for (dy, dx) in directions {
                if (0..4).all(|i| {
                    let new_row = row + i * dy;
                    let new_col = col + i * dx;

                    new_row >= 0
                        && new_col >= 0
                        && new_row < rows
                        && new_col < cols
                        && grid[new_row as usize][new_col as usize] == target[i as usize]
                }) {
                    count += 1;
                }
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
