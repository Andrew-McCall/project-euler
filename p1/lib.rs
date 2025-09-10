/* p1: Multiples of 3 and 5 */
pub fn solution(limit: usize) -> usize {
    let mut sum = 0;
    for i in 1..limit {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }
    sum
}

#[test]
fn test() {
    assert_eq!(solution(10), 23);
    assert_eq!(solution(1000), 233168);
}

// TODO: Lookup Rust magic numbers/branchless solution
