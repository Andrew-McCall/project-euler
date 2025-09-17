/* p15: Lattice Paths */
pub fn solution(n: u64) -> u64 {
    // Binomial Coefficient (Factorials were too big)
    let mut result = 1;
    for i in 1..=n {
        result = result * (n + i) / i;
    }
    result
}

#[test]
fn test() {
    assert_eq!(solution(2), 6);
    assert_eq!(solution(20), 137846528820);
}
