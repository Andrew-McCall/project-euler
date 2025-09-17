/* p10: Summation of Primes */
pub fn solution(n: u64) -> u64 {
    match n {
        1 | 0 => return 0,
        2 => return 2,
        3 => return 5,
        4 => return 5,
        5 => return 10,
        _ => {}
    };

    let mut sum = 5;

    for x in (5..n).step_by(2) {
        if is_prime(x) {
            sum += x;
        }
    }

    sum
}

#[test]
fn test() {
    assert_eq!(solution(10), 17);
    assert_eq!(solution(2000000), 142913828922);
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    let sqrt_n = (n as f64).sqrt() as u64;
    for i in (3..=sqrt_n).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}
