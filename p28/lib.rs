pub fn solution(size: u64) -> u64 {
    let mut sum = 1;
    let mut n = 3;

    while n <= size {
        let step = n - 1;
        sum += 4 * n * n - 6 * step;
        n += 2;
    }

    sum
}

#[test]
fn test() {
    assert_eq!(solution(1), 1);
    assert_eq!(solution(5), 101);
    assert_eq!(solution(1001), 669171001);
}
