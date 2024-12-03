use aoc2024::aoc;
use std::collections::HashMap;

fn main() {
    let lines = aoc::get_input(1);

    let time_part1 = std::time::Instant::now();
    let result_part1 = part1(&lines);
    let duration_part1 = time_part1.elapsed();
    println!("Part 1 ({:?}): {:?}", duration_part1, result_part1);

    let time_part2 = std::time::Instant::now();
    let result_part2 = part2(&lines);
    let duration_part2 = time_part2.elapsed();
    println!("Part 2 ({:?}): {:?}", duration_part2, result_part2);
}

fn part1(content: &[String]) -> u32 {
    let (left, right) = get_nums(content);

    let difference = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum();

    difference
}

fn part2(content: &[String]) -> u32 {
    let (left, right) = get_nums(content);

    let mut right_counts: HashMap<u32, u32> = HashMap::new();
    for &num in &right {
        *right_counts.entry(num).or_insert(0) += 1;
    }

    let similarity: u32 = left
        .iter()
        .map(|&num| num * right_counts.get(&num).cloned().unwrap_or(0))
        .sum();

    similarity
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

#[cfg(test)]
mod tests {
    use super::*;
    use aoc2024::aoc::to_lines;
    use indoc::indoc;

    #[test]
    fn test_part1() {
        let content = indoc! {"
            3   4
            4   3
            2   5
            1   3
            3   9
            3   3
        "};
        assert_eq!(part1(&to_lines(content)), 11);
        assert_eq!(part2(&to_lines(content)), 31);
    }
}
