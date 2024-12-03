use aoc2024::utils;
use regex::Regex;

fn main() {
    let input = utils::read_input(3);

    println!("Part 1: {}", solve_part1(&input));
    println!("Part 2: {}", solve_part2(&input));
}

// TODO: regex maybe? because WTF have i done

fn solve_part1(input: &str) -> i32 {
    let mut total = 0;
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c == 'm'
            && chars.next() == Some('u')
            && chars.next() == Some('l')
            && chars.next() == Some('(')
        {
            let mut num1 = String::new();
            let mut num2 = String::new();
            let mut comma_found = false;

            while let Some(&next) = chars.peek() {
                if next.is_digit(10) {
                    if !comma_found {
                        num1.push(chars.next().unwrap());
                    } else {
                        num2.push(chars.next().unwrap());
                    }
                } else if next == ',' {
                    if comma_found {
                        break;
                    }
                    comma_found = true;
                    chars.next();
                } else if next == ')' {
                    chars.next();

                    if !num1.is_empty() && !num2.is_empty() {
                        let x = num1.parse::<i32>().unwrap();
                        let y = num2.parse::<i32>().unwrap();
                        total += x * y;
                    }
                    break;
                } else {
                    break;
                }
            }
        }
    }

    total
}

// TODO: regex crate. not sure if this the proper way to handle this?
// ask rust ment. then refac part 1

fn solve_part2(input: &str) -> i32 {
    let p = Regex::new(r"mul\((\d+),(\d+)\)|(do|don't)\(\)").unwrap();

    let mut total = 0;
    let mut mul_enabled = true;

    for line in input.lines() {
        for cap in p.captures_iter(line) {
            if let Some(mul_group) = cap.get(0) {
                let mul_str = mul_group.as_str();

                if mul_str.starts_with("mul") {
                    let x: i32 = cap[1].parse().unwrap();
                    let y: i32 = cap[2].parse().unwrap();
                    if mul_enabled {
                        total += x * y;
                    }
                } else {
                    mul_enabled = !cap[0].starts_with("don't");
                }
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
