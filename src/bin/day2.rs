use aoc2024::aoc;
#[cfg(test)]
use aoc2024::aoc::to_lines;

fn main() {
    let lines = aoc::get_input(2);

    let time_part1 = std::time::Instant::now();
    let result_part1 = part1(&lines);
    let duration_part1 = time_part1.elapsed();
    println!("Part 1 ({:?}): {:?}", duration_part1, result_part1);

    let time_part2 = std::time::Instant::now();
    let result_part2 = part2(&lines);
    let duration_part2 = time_part2.elapsed();
    println!("Part 2 ({:?}): {:?}", duration_part2, result_part2);
}

fn part1(reports: &Vec<String>) -> usize {
    let safe_reports = reports
        .iter()
        .map(parse_report)
        .filter(|report| is_safe_no_removals(&report));

    safe_reports.count()
}

fn part2(reports: &Vec<String>) -> usize {
    let safe_reports = reports
        .iter()
        .map(parse_report)
        .filter(|report| is_safe_no_removals(&report) || is_safe_with_removal(&report));

    safe_reports.count()
}

fn parse_report(report: &String) -> Vec<u32> {
    report
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn is_safe_no_removals(report: &[u32]) -> bool {
    let is_increasing = report[0] != report[1] && report[0] <= report[1];
    for (prev, curr) in report.windows(2).map(|p| (p[0], p[1])) {
        let diff = curr as i32 - prev as i32; // increasing, this should be +, decreasing, this should be -
        match (is_increasing, diff) {
            (true, d) if d < 0 => return false,
            (false, d) if d > 0 => return false,
            (_, d) if d.abs() < 1 || d.abs() > 3 => return false,
            _ => continue,
        }
    }
    true
}

fn is_safe_with_removal(report: &Vec<u32>) -> bool {
    for (i, _) in report.iter().enumerate() {
        let mut report = report.clone();
        report.remove(i);
        if is_safe_no_removals(&report) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part1() {
        let content = indoc! {"
            7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9
        "};
        assert_eq!(part1(&to_lines(content)), 2);
        assert_eq!(part2(&to_lines(content)), 4);
    }
}
