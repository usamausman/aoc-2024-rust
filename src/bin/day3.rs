use aoc2024::aoc;
use regex::Regex;
use std::sync::LazyLock;

fn main() {
    let lines = aoc::get_input(3);
    let memory = lines.join(" ");

    let time_part1 = std::time::Instant::now();
    let result_part1 = part1(&memory);
    let duration_part1 = time_part1.elapsed();
    println!("Part 1 ({:?}): {:?}", duration_part1, result_part1);

    let time_part2 = std::time::Instant::now();
    let result_part2 = part2(&memory);
    let duration_part2 = time_part2.elapsed();
    println!("Part 2 ({:?}): {:?}", duration_part2, result_part2);
}

fn part1(memory: &str) -> u32 {
    follow_instructions(memory).iter().sum()
}

fn part2(memory: &str) -> u32 {
    follow_enabled_instructions(memory).iter().sum()
}

static MUL_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"mul\((\d+),(\d+)\)").unwrap());

fn follow_instructions(memory: &str) -> Vec<u32> {
    MUL_REGEX
        .captures_iter(memory)
        .map(|c| {
            let a: u32 = c[1].parse().unwrap();
            let b: u32 = c[2].parse().unwrap();
            a * b
        })
        .collect()
}

fn follow_enabled_instructions(memory: &str) -> Vec<u32> {
    // Split on `don't()`
    memory
        .split("don't()")
        .enumerate()
        .map(|(i, l)| {
            // First is enabled by default, later ones are disabled until we see a `do()`
            if i == 0 {
                l
            } else {
                l.splitn(2, "do()").nth(1).unwrap_or("")
            }
        })
        .flat_map(follow_instructions)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let content = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(part1(&content), 161);
    }

    #[test]
    fn test_part2() {
        let content = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(part2(&content), 48);
    }
}
