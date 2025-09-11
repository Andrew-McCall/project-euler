/* p16: Power Digit Sum */
pub fn solution(base: u64, power: u64) -> u64 {
    let mut digits: Vec<u64> = vec![1];
    let mut carry: u64 = 0;

    for _ in 0..power {
        for d in &mut digits {
            let current = *d * base + carry;
            carry = current / 10;
            *d = current % 10;
        }
        while carry > 0 {
            digits.push(carry % 10);
            carry /= 10
        }
    }

    // Doesn't matter for sum but digits are in reverse order 16 = [6,1]
    digits.iter().sum()
}

#[test]
fn test() {
    assert_eq!(solution(2, 4), 7);
    assert_eq!(solution(3, 3), 9);
    assert_eq!(solution(10, 10000), 1);
    // assert_eq!(solution(17, 100000), 551881);
    assert_eq!(solution(2, 1000), 1366);
}
