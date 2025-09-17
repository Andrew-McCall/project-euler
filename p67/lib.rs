use std::{fs::File, io::Read};

/* p0: Example */
pub fn solution(path: &str) -> u64 {
    p18::solution(read_input_file(path))
}

pub const INPUT_PATH: &str = "0067_triangle.txt";

pub fn read_input_file(path: &str) -> Vec<u64> {
    let mut text_file = File::open(path).expect("Failed to open file");
    let mut contents = String::new();
    text_file
        .read_to_string(&mut contents)
        .expect("Unable to parse file");
    contents
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect()
}

#[test]
fn test() {
    assert_eq!(solution(INPUT_PATH), 0);
}
