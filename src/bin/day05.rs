use aoc2024::utils;

fn main() {
    let input = utils::read_input(5);

    println!("Part 1: {}", solve_part1(&input));
    println!("Part 2: {}", solve_part2(&input));
}

fn solve_part1(input: &str) -> i32 {
    let sections: Vec<&str> = input.split("\n\n").collect();

    let mut x_vals: Vec<i32> = Vec::new();
    let mut y_vals: Vec<i32> = Vec::new();

    if let Some(section1) = sections.get(0) {
        section1.lines().for_each(|line| {
            let mut parts = line.split("|");
            let x = parts.next().unwrap().parse::<i32>().unwrap();
            let y = parts.next().unwrap().parse::<i32>().unwrap();
            x_vals.push(x);
            y_vals.push(y);
        });
    }

    let mut valid_middle_pages = 0;

    if let Some(section2) = sections.get(1) {
        for line in section2.lines() {
            let update: Vec<i32> = line
                .split(",")
                .map(|page| page.parse::<i32>().unwrap())
                .collect();

            let mut is_valid = true;

            for i in 0..x_vals.len() {
                let x = x_vals[i];
                let y = y_vals[i];

                if let (Some(index_x), Some(index_y)) = (
                    update.iter().position(|&p| p == x),
                    update.iter().position(|&p| p == y),
                ) {
                    if index_x > index_y {
                        is_valid = false;
                        break;
                    }
                }
            }

            if is_valid {
                valid_middle_pages += update[update.len() / 2];
            }
        }
    }

    valid_middle_pages
}

fn solve_part2(input: &str) -> i32 {
    let sections: Vec<&str> = input.split("\n\n").collect();

    let mut x_vals: Vec<i32> = Vec::new();
    let mut y_vals: Vec<i32> = Vec::new();

    if let Some(section1) = sections.get(0) {
        section1.lines().for_each(|line| {
            let mut parts = line.split("|");
            let x = parts.next().unwrap().parse::<i32>().unwrap();
            let y = parts.next().unwrap().parse::<i32>().unwrap();
            x_vals.push(x);
            y_vals.push(y);
        });
    }

    let mut valid_middle_pages = 0;

    if let Some(section2) = sections.get(1) {
        for line in section2.lines() {
            let mut update: Vec<i32> = line
                .split(",")
                .map(|page| page.parse::<i32>().unwrap())
                .collect();

            let mut is_valid = true;

            for i in 0..x_vals.len() {
                let x = x_vals[i];
                let y = y_vals[i];

                if let (Some(index_x), Some(index_y)) = (
                    update.iter().position(|&p| p == x),
                    update.iter().position(|&p| p == y),
                ) {
                    if index_x > index_y {
                        is_valid = false;
                        break;
                    }
                }
            }

            if !is_valid {
                update.sort_by(|a, b| {
                    for i in 0..x_vals.len() {
                        let x = x_vals[i];
                        let y = y_vals[i];

                        if *a == x && *b == y {
                            return std::cmp::Ordering::Less;
                        } else if *a == y && *b == x {
                            return std::cmp::Ordering::Greater;
                        }
                    }
                    std::cmp::Ordering::Equal
                });

                valid_middle_pages += update[update.len() / 2];
            }
        }
    }

    valid_middle_pages
}
#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

    #[test]
    fn part1() {
        let result = super::solve_part1(SAMPLE_INPUT);
        assert_eq!(result, 143)
    }

    #[test]
    fn part2() {
        let result = super::solve_part2(SAMPLE_INPUT);
        assert_eq!(result, 123)
    }
}
