use std::fs;

pub fn get_input(day: u32) -> Vec<String> {
    let string = fs::read_to_string(format!("input/day_{}/in", day)).unwrap();
    to_lines(&string)
}

pub fn to_lines(content: &str) -> Vec<String> {
    content.lines().map(|l| l.to_owned()).collect()
}
