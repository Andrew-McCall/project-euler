/* p20: Factorial Digit Sum */
pub fn solution(n: u64) -> u64 {
    let mut digits: Vec<u64> = vec![1];
    let mut carry: u64 = 0;

    for i in 1..n {
        for d in &mut digits {
            let current = *d * i + carry;
            carry = current / 10;
            *d = current % 10;
        }
        while carry > 0 {
            digits.push(carry % 10);
            carry /= 10
        }
    }

    digits.iter().sum()
}

#[test]
fn test() {
    assert_eq!(solution(10), 27);
    assert_eq!(solution(100), 648);
}

// Similar to p16
