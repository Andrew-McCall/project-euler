/* p2: Even Fibonacci Numbers */
pub fn solution(limit: u32) -> u64 {
    let mut result: u64 = 0;
    let mut a = 0;
    let mut b = 1;

    while a < limit {
        if a & 1 == 0 {
            result += a as u64;
        }
        let next = a + b;
        a = b;
        b = next;
    }
    result
}

#[test]
fn test() {
    assert_eq!(solution(10), 10);
    assert_eq!(solution(4000000), 4613732);
}
