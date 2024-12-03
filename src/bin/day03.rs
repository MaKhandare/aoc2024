use aoc2024::utils;
use regex::Regex;

fn main() {
    let input = utils::read_input(3);

    println!("Part 1: {}", solve_part1(&input));
    println!("Part 2: {}", solve_part2(&input));
}

// REGEX!
// check for mul(x,y), where x and y have 1-3 digits
// the digits are in groups. get groups with captures[1] and captures[2]

fn solve_part1(input: &str) -> i32 {
    let mut total = 0;
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    for captures in re.captures_iter(input) {
        let x: i32 = captures[1].parse().unwrap();
        let y: i32 = captures[2].parse().unwrap();
        total += x * y;
    }

    total
}

// REGEX!
// check for do(), don't() and mul(x,y)
// if do() enable mul
// if don't() disable mul
// mul stays the same essentially

fn solve_part2(input: &str) -> i32 {
    let mut total = 0;
    let mut mul_enabled = true;

    let re = Regex::new(r"do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    for captures in re.captures_iter(input) {
        if let Some(command) = captures.get(0) {
            match command.as_str() {
                "do()" => {
                    mul_enabled = true;
                }
                "don't()" => {
                    mul_enabled = false;
                }
                _ if mul_enabled => {
                    let x: i32 = captures[1].parse().unwrap();
                    let y: i32 = captures[2].parse().unwrap();
                    total += x * y;
                }
                _ => {}
            }
        }
    }
    total
}
#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str =
        r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const SAMPLE_INPUT2: &str =
        r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn part1() {
        let result = super::solve_part1(SAMPLE_INPUT);
        assert_eq!(result, 161)
    }

    #[test]
    fn part2() {
        let result = super::solve_part2(SAMPLE_INPUT2);
        assert_eq!(result, 48)
    }
}
