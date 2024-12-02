use aoc2024::aoc;
#[cfg(test)]
use aoc2024::aoc::to_lines;
use std::collections::HashMap;

fn main() {
    let lines = aoc::get_input(1);

    let time_part1 = std::time::Instant::now();
    println!("Part 1: {:?}", part1(&lines).unwrap());
    eprintln!("# Took {:.2?}", time_part1.elapsed());

    let time_part2 = std::time::Instant::now();
    println!("Part 2: {:?}", part2(&lines).unwrap());
    eprintln!("# Took {:.2?}", time_part2.elapsed());
}

fn part1(content: &[String]) -> Option<u32> {
    let (left, right) = get_nums(content);

    let difference = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum();

    Some(difference)
}

fn part2(content: &[String]) -> Option<u32> {
    let (left, right) = get_nums(content);

    let mut right_counts: HashMap<u32, u32> = HashMap::new();
    for &num in &right {
        *right_counts.entry(num).or_insert(0) += 1;
    }

    let similarity: u32 = left
        .iter()
        .map(|&num| num * right_counts.get(&num).cloned().unwrap_or(0))
        .sum();

    Some(similarity)
}

fn get_nums(lines: &[String]) -> (Vec<u32>, Vec<u32>) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in lines {
        let mut numbers = line.split_whitespace().map(|x| x.parse::<u32>().unwrap());
        left.push(numbers.next().unwrap());
        right.push(numbers.next().unwrap());
    }

    left.sort();
    right.sort();

    (left, right)
}

#[test]
fn test_part1() {
    let content = "3   4
    4   3
    2   5
    1   3
    3   9
    3   3";
    assert_eq!(part1(&to_lines(content)), Some(11));
}

#[test]
fn test_part2() {
    let content = "3   4
    4   3
    2   5
    1   3
    3   9
    3   3";
    assert_eq!(part2(&to_lines(content)), Some(31));
}
