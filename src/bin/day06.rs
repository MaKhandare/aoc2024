use std::collections::HashSet;

use aoc2024::utils;

fn main() {
    let input = utils::read_input(6);

    println!("Part 1: {}", solve_part1(&input));
    println!("Part 2: {}", solve_part2(&input));
}

fn solve_part1(input: &str) -> i32 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let rows = grid.len();
    let cols = grid[0].len();

    let mut visited = HashSet::new();

    let mut guard_pos = (0, 0);

    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] == '^' {
                guard_pos = (row, col);
                visited.insert(guard_pos);
                break;
            }
        }
        if guard_pos != (0, 0) {
            break;
        }
    }

    let mut direction_index = 0;
    let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    loop {
        let next_pos = (
            guard_pos.0 as i32 + directions[direction_index].0,
            guard_pos.1 as i32 + directions[direction_index].1,
        );

        if next_pos.0 < 0
            || next_pos.0 >= rows as i32
            || next_pos.1 < 0
            || next_pos.1 >= cols as i32
            || grid[next_pos.0 as usize][next_pos.1 as usize] == '#'
        {
            direction_index = (direction_index + 1) % 4;
        } else {
            guard_pos = (next_pos.0 as usize, next_pos.1 as usize);
            visited.insert(guard_pos);
        }

        // pfiat di
        if next_pos.0 < 0
            || next_pos.0 >= rows as i32
            || next_pos.1 < 0
            || next_pos.1 >= cols as i32
        {
            break;
        }
    }

    visited.len() as i32
}

fn solve_part2(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

    #[test]
    fn part1() {
        let result = super::solve_part1(SAMPLE_INPUT);
        assert_eq!(result, 41)
    }

    #[test]
    fn part2() {
        let result = super::solve_part2(SAMPLE_INPUT);
        assert_eq!(result, 123)
    }
}
