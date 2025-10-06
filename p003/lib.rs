/* p3: Largest Prime Factor  */
pub fn solution(mut n: u64) -> u64 {
    let mut max_factor = 1;

    // Handle factor 2
    if n.is_multiple_of(2) {
        max_factor = 2;
    }

    // Check odd numbers only
    let mut f = 3;
    while f * f <= n {
        if n.is_multiple_of(f) && is_prime(f) {
            max_factor = f;
            while n.is_multiple_of(f) {
                n /= f;
            }
        }
        f += 2;
    }

    // If leftover > 1 and prime
    if n > 1 && is_prime(n) {
        return n;
    }

    max_factor
}

#[test]
fn test() {
    assert_eq!(solution(17), 17);
    assert_eq!(solution(100), 5);
    assert_eq!(solution(600851475143), 6857);
}

#[test]
fn test_is_prime() {
    assert!(is_prime(2));
    assert!(is_prime(5));
    assert!(is_prime(17));
    assert!(is_prime(19));
    assert!(!is_prime(21));
    assert!(!is_prime(22421));
    assert!(!is_prime(22422));
}

// Miller–Rabin
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

    for i in (3..=(n as f64).sqrt() as u64).step_by(2) {
        if n.is_multiple_of(i) {
            return false;
        }
    }
    true
}

// fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
//     let mut result: u64 = 1;
//     base %= modulus;
//     while exp > 0 {
//         if exp % 2 == 1 {
//             result = result.wrapping_mul(base) % modulus;
//         }
//         base = base.wrapping_mul(base) % modulus;
//         exp /= 2;
//     }
//     result
// }
