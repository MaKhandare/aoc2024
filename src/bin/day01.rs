use std::collections::HashMap;

use aoc2024::utils;

fn main() {
    let input = utils::read_input(1);

    println!("Part 1: {}", solve_part1(&input));
    println!("Part 2: {}", solve_part2(&input));
}

fn solve_part1(input: &str) -> i32 {
    let mut l: Vec<i32> = Vec::new();
    let mut r: Vec<i32> = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();

        l.push(parts[0].parse::<i32>().unwrap());
        r.push(parts[1].parse::<i32>().unwrap());
    }

    l.sort();
    r.sort();

    let mut total_distance = 0;
    for (left, right) in l.iter().zip(r.iter()) {
        total_distance += (left - right).abs()
    }

    total_distance
}

fn solve_part2(input: &str) -> i32 {
    let mut l: Vec<i32> = Vec::new();
    let mut r: Vec<i32> = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();

        l.push(parts[0].parse::<i32>().unwrap());
        r.push(parts[1].parse::<i32>().unwrap());
    }

    l.sort();

    let mut map: HashMap<i32, i32> = HashMap::new();

    for num in r.iter() {
        *map.entry(*num).or_insert(0) += 1;
    }

    let mut similarity_score = 0;
    for left in l.iter() {
        let count = *map.get(left).unwrap_or(&0);
        similarity_score += left * count;
    }

    similarity_score
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
