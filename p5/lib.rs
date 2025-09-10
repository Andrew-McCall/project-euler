/* p5: Smallest Multiple */
pub fn solution(bound: u64) -> u64 {
    (1..=bound).reduce(least_common_multiple).unwrap()
}

#[test]
pub fn test() {
    assert_eq!(solution(10), 2520);
    assert_eq!(solution(20), 232792560);
}

fn greatest_common_divisor(x: u64, y: u64) -> u64 {
    if y == 0 {
        x
    } else {
        greatest_common_divisor(y, x % y)
    }
}

fn least_common_multiple(x: u64, y: u64) -> u64 {
    x / greatest_common_divisor(x, y) * y
}
