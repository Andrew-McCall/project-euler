/* p6: Sum Square Difference */
pub fn solution(n: u64) -> u64 {
    let sum = (n * (n + 1)) / 2;
    let sum_of_squares = (n * (n + 1) * (2 * n + 1)) / 6;
    sum * sum - sum_of_squares
}

pub fn alt_solution(n: u64) -> u64 {
    let sum = (1..=n).sum::<u64>();
    let sum_of_squares = (1..=n).map(|x| x * x).sum::<u64>();
    sum * sum - sum_of_squares
}

#[test]
fn test() {
    assert_eq!(alt_solution(10), 2640);
    assert_eq!(solution(10), 2640);
    assert_eq!(alt_solution(100), 25164150);
    assert_eq!(solution(100), 25164150);
}
