use std::collections::HashMap;
use std::fs;

fn main() {
    let content = fs::read_to_string("input/day_1/in").unwrap();
    let lines = to_lines(content.as_str());

    let time_part1 = std::time::Instant::now();
    println!("Part 1: {:?}", part1(&lines).unwrap());
    eprintln!("# Took {:.2?}", time_part1.elapsed());

    let time_part2 = std::time::Instant::now();
    println!("Part 2: {:?}", part2(&lines).unwrap());
    eprintln!("# Took {:.2?}", time_part2.elapsed());
}

fn to_lines(content: &str) -> Vec<&str> {
    let lines: Vec<&str> = content.lines().collect();
    lines
}

fn part1(content: &[&str]) -> Option<i32> {
    let (left, right) = get_nums(content);

    let difference = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum();

    Some(difference)
}

fn part2(content: &[&str]) -> Option<i32> {
    let (left, right) = get_nums(content);

    let mut right_counts: HashMap<i32, i32> = HashMap::new();
    for &num in &right {
        *right_counts.entry(num).or_insert(0) += 1;
    }

    let similarity: i32 = left
        .iter()
        .map(|&num| num * right_counts.get(&num).cloned().unwrap_or(0))
        .sum();

    Some(similarity)
}

fn get_nums(lines: &[&str]) -> (Vec<i32>, Vec<i32>) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in lines {
        let mut numbers = line.split_whitespace().map(|x| x.parse::<i32>().unwrap());
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
