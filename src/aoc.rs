use std::fs;

pub fn get_input(day: u32) -> Vec<String> {
    let string = fs::read_to_string(format!("input/day_{}/in", day)).unwrap();
    let content = string.as_str();
    to_lines(content)
}

pub fn to_lines(content: &str) -> Vec<String> {
    content.lines().map(|x| x.to_string()).collect()
}
