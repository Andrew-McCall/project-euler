/* p7: 10001st Prime */
pub fn solution(n: u64) -> u64 {
    let sieve_bound = upper_bound_nth_prime(n);
    let mut primes: Vec<u64> = Vec::new();

    for x in 2..sieve_bound {
        if is_prime(x) {
            primes.push(x);
            if primes.len() == n as usize {
                return x;
            }
        }
    }

    unreachable!()
}

#[test]
fn test() {
    assert_eq!(solution(10), 29);
    assert_eq!(solution(100), 541);
    assert_eq!(solution(10001), 104743);
}

fn upper_bound_nth_prime(n: u64) -> u64 {
    let n = n as f64;
    (n * (n.ln() + n.ln().ln())).ceil() as u64
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n.is_multiple_of(2) {
        return false;
    }

    let sqrt_n = (n as f64).sqrt() as u64;
    for i in (3..=sqrt_n).step_by(2) {
        if n.is_multiple_of(i) {
            return false;
        }
    }
    true
}
