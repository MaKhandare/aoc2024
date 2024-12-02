use std::collections::HashMap;

use aoc2024::utils;

fn main() {
    let input = utils::read_input(1);

    println!("Part 1: {}", solve_part1(&input));
    println!("Part 2: {}", solve_part2(&input));
}

fn solve_part1(input: &str) -> i32 {
    let (mut l, mut r): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            (
                parts.next().unwrap().parse::<i32>().unwrap(),
                parts.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .unzip();

    l.sort();
    r.sort();

    l.iter()
        .zip(r.iter())
        .map(|(left, right)| (left - right).abs())
        .sum()
}

fn solve_part2(input: &str) -> i32 {
    let (mut l, r): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            (
                parts.next().unwrap().parse::<i32>().unwrap(),
                parts.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .unzip();

    l.sort();

    let mut map: HashMap<i32, i32> = HashMap::new();
    for num in r.iter() {
        *map.entry(*num).or_insert(0) += 1;
    }

    l.iter()
        .map(|left| left * map.get(&left).unwrap_or(&0))
        .sum()
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = r"3   4
    4   3
    2   5
    1   3
    3   9
    3   3";

    #[test]
    fn part1() {
        let result = super::solve_part1(SAMPLE_INPUT);
        assert_eq!(result, 11)
    }

    #[test]
    fn part2() {
        let result = super::solve_part2(SAMPLE_INPUT);
        assert_eq!(result, 31)
    }
}
