/* p36: Double-base Palindromes */
pub fn solution(bound: u64) -> u64 {
    let mut sum = 0;

    for n in 1..bound {
        if is_base10_palindrome(n) && is_base2_palindrome(n) {
            sum += n;
        }
    }

    sum
}

#[test]
fn test() {
    assert_eq!(solution(1_000), 1772);
    assert_eq!(solution(1_000_000), 872187);
}

fn is_base10_palindrome(mut n: u64) -> bool {
    if n < 10 {
        return true;
    }
    let original = n;
    let mut reversed = 0;
    while n > 0 {
        reversed = reversed * 10 + n % 10;
        n /= 10;
    }
    original == reversed
}

fn is_base2_palindrome(mut n: u64) -> bool {
    let original = n;
    if n == 0 {
        return true;
    }
    let bit_len = 64 - n.leading_zeros();
    let mut reversed = 0;
    for _ in 0..bit_len {
        reversed = (reversed << 1) | (n & 1);
        n >>= 1;
    }
    reversed == original
}
