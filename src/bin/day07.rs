use aoc2024::utils;

fn main() {
    let input = utils::read_input(7);

    println!("Part 1: {}", solve_part1(&input));
    println!("Part 2: {}", solve_part2(&input));
}

fn solve_part1(input: &str) -> u64 {
    let mut total_calibrations = 0;

    for line in input.lines() {
        let parts: Vec<&str> = line.split(":").collect();

        let target: u64 = parts[0].trim().parse::<u64>().unwrap();
        let numbers: Vec<u64> = parts[1]
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        if is_solveable(target, &numbers, false) {
            total_calibrations += target
        }
    }

    total_calibrations
}

fn solve_part2(input: &str) -> u64 {
    let mut total_calibrations = 0;

    for line in input.lines() {
        let parts: Vec<&str> = line.split(":").collect();

        let target: u64 = parts[0].trim().parse::<u64>().unwrap();
        let numbers: Vec<u64> = parts[1]
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        if is_solveable(target, &numbers, true) {
            total_calibrations += target
        }
    }

    total_calibrations
}

fn is_solveable(target: u64, numbers: &[u64], concat_mode: bool) -> bool {
    fn helper(current: u64, index: usize, numbers: &[u64], target: u64, concat_mode: bool) -> bool {
        if index == numbers.len() {
            return current == target;
        }

        if helper(
            current + numbers[index],
            index + 1,
            numbers,
            target,
            concat_mode,
        ) {
            return true;
        }

        if helper(
            current * numbers[index],
            index + 1,
            numbers,
            target,
            concat_mode,
        ) {
            return true;
        }

        if concat_mode && index + 1 <= numbers.len() {
            let concatenated = concatenate(current, numbers[index]);
            if helper(concatenated, index + 1, numbers, target, concat_mode) {
                return true;
            }
        }

        false
    }

    helper(numbers[0], 1, numbers, target, concat_mode)
}

// nice
// https://www.reddit.com/r/rust/comments/191l3ot/concatinate_two_numbers/
fn concatenate(left: u64, right: u64) -> u64 {
    let right_digits = right.to_string().len();
    left * 10_u64.pow(right_digits as u32) + right
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

    #[test]
    fn part1() {
        let result = super::solve_part1(SAMPLE_INPUT);
        assert_eq!(result, 3749)
    }

    #[test]
    fn part2() {
        let result = super::solve_part2(SAMPLE_INPUT);
        assert_eq!(result, 11387)
    }
}
