/* p25: 1000-digit Fibonacci Number */
// Binet’s formula
pub fn solution(digits: u64) -> u64 {
    let sqrt_5 = 5.0_f64.sqrt();
    let phi = (1.0_f64 + sqrt_5) / 2.0_f64;

    let n = ((digits as f64 - 1.0) + sqrt_5.log10()) / phi.log10();
    n.ceil() as u64
}

#[test]
fn test() {
    assert_eq!(solution(1_000), 4782);

    // fun
    assert_eq!(solution(1_000_000), 4784969);
    assert_eq!(solution(1_000_000_000), 4784971964);
    assert_eq!(solution(1_000_000_000_000), 4784971966779);
}
