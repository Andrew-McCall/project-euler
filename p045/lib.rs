/* p45: Triangular, Pentagonal, and Hexagonal */
pub fn solution(lower: u64, upper: u64) -> u64 {
    assert!(lower > 0);

    let triangle: Vec<u64> = (lower..=upper).map(|n| n * (n + 1) / 2).collect();
    let hexagonal: Vec<u64> = (lower..=upper).map(|n| n * (2 * n - 1)).collect();
    let pentagonal: Vec<u64> = (lower..=upper).map(|n| n * (3 * n - 1) / 2).collect();

    for t in triangle {
        if hexagonal.binary_search(&t).is_ok() && pentagonal.binary_search(&t).is_ok() {
            return t;
        }
    }

    0
}

#[test]
fn test() {
    assert_eq!(solution(287, 100_000), 40755);
}
