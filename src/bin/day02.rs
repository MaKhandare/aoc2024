use aoc2024::utils;

fn main() {
    let input = utils::read_input(2);

    println!("Part 1: {}", solve_part1(&input));
    println!("Part 2: {}", solve_part2(&input));
}

fn solve_part1(input: &str) -> i32 {
    input
        .lines()
        .filter(|line| {
            let levels: Vec<i32> = line
                .split_whitespace()
                .map(|part| part.parse::<i32>().unwrap())
                .collect();
            is_safe(&levels)
        })
        .count() as i32
}

fn solve_part2(input: &str) -> i32 {
    input
        .lines()
        .filter(|line| {
            let levels: Vec<i32> = line
                .split_whitespace()
                .map(|part| part.parse::<i32>().unwrap())
                .collect();

            if is_safe(&levels) {
                return true;
            }

            for i in 0..levels.len() {
                let mut tmp = levels.clone();
                tmp.remove(i);
                if is_safe(&tmp) {
                    return true;
                }
            }

            false
        })
        .count() as i32
}

fn is_safe(levels: &Vec<i32>) -> bool {
    let mut is_increasing = true;
    let mut is_decreasing = true;

    for i in 0..levels.len() - 1 {
        let diff = levels[i + 1] - levels[i];
        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }

        if diff < 0 {
            is_increasing = false;
        } else if diff > 0 {
            is_decreasing = false;
        }
    }

    is_increasing || is_decreasing
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = r"7 6 4 2 1
    1 2 7 8 9
    9 7 6 2 1
    1 3 2 4 5
    8 6 4 4 1
    1 3 6 7 9";

    #[test]
    fn part1() {
        let result = super::solve_part1(SAMPLE_INPUT);
        assert_eq!(result, 2)
    }

    #[test]
    fn part2() {
        let result = super::solve_part2(SAMPLE_INPUT);
        assert_eq!(result, 4)
    }
}
