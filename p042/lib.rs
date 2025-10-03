/* p42: Coded Triangle Numbers */
pub fn solution(words: Vec<String>) -> u64 {
    let mut count = 0;

    for word in words {
        let sum = word.as_bytes().iter().fold(0_u64, |sum, char| {
            if char.is_ascii_alphabetic() {
                sum + ((char.to_ascii_lowercase() - b'a') as u64) + 1
            } else {
                sum
            }
        });

        if is_triangle(sum) {
            count += 1;
        }
    }

    count
}

#[test]
fn test() {
    assert_eq!(solution(file_to_words(EXAMPLE_FILE)), 162);
}

// x = (t * t + t )/2
pub fn is_triangle(n: u64) -> bool {
    let discriminant = 1 + 8 * n;
    let sqrt_d = (discriminant as f64).sqrt();
    sqrt_d.fract() == 0.0 && ((-1.0 + sqrt_d) / 2.0) >= 0.0
}

pub const EXAMPLE_FILE: &str = "0042_words.txt";

pub fn file_to_words(path: &str) -> Vec<String> {
    let contents = std::fs::read_to_string(path).expect("Failed to read file");
    contents
        .trim_matches('"')
        .split(r#"",""#)
        .map(|s| s.to_string())
        .collect()
}
