/* p22: Names Scores */
pub fn solution(names: Vec<String>) -> u64 {
    let mut name_worths = names
        .iter()
        .map(|n| n.chars().map(|n| get_worth(n) as u64).collect::<Vec<u64>>())
        .collect::<Vec<_>>();

    name_worths.sort();

    name_worths
        .iter()
        .enumerate()
        .map(|worth| (worth.1.iter().sum::<u64>()) * (worth.0 as u64 + 1))
        .sum()
}

pub const INPUT_PATH: &str = "0022_names.txt";

pub fn file_to_names(path: &str) -> Vec<String> {
    let contents = std::fs::read_to_string(path).expect("Failed to read file");
    contents
        .trim_matches('"')
        .split(r#"",""#)
        .map(|s| s.to_string())
        .collect()
}

#[test]
fn test() {
    assert_eq!(solution(file_to_names(INPUT_PATH)), 0);
}

fn get_worth(letter: char) -> u8 {
    match letter {
        'A'..='Z' => (letter as u8) - b'A' + 1,
        _ => unreachable!(),
    }
}
