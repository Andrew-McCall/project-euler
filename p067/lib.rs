/* p67: Maximum Path Sum II */
pub fn solution(path: &str) -> u64 {
    p18::solution(read_input_file(path))
}

pub const INPUT_PATH: &str = "0067_triangle.txt";

pub fn read_input_file(path: &str) -> Vec<u64> {
    let contents = std::fs::read_to_string(path).expect("Failed to read file");
    contents
        .split_whitespace()
        .map(|n| n.parse::<u64>().expect("Failed to parse number"))
        .collect()
}

#[test]
fn test() {
    assert_eq!(solution(INPUT_PATH), 7273);
}
