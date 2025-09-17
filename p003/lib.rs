/* p3: Largest Prime Factor  */
pub fn solution(mut n: u64) -> u64 {
    let mut max_factor = 1;

    // Handle factor 2
    if n % 2 == 0 {
        if is_prime(2) {
            max_factor = 2;
        }
        while n % 2 == 0 {
            n /= 2;
        }
    }

    // Check odd numbers only
    let mut f = 3;
    while f * f <= n {
        if n % f == 0 && is_prime(f) {
            max_factor = f;
            while n % f == 0 {
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
    match n {
        0 => return false,
        1 => return false,
        2 => return true,
        3 => return true,
        n if n % 2 == 0 => return false,
        _ => {}
    }

    // Write n-1 as 2^s * d
    let mut d = n - 1;
    let mut s = 0;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }

    let bases: [u64; 8] = [2, 3, 5, 7, 11, 13, 17, 19];

    'outer: for &a in &bases {
        if a >= n {
            continue;
        }
        let mut x = mod_pow(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 0..s - 1 {
            x = mod_pow(x, 2, n);
            if x == n - 1 {
                continue 'outer;
            }
        }
        return false; // composite
    }

    true // probably prime
}

fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    let mut result: u64 = 1;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result.wrapping_mul(base) % modulus;
        }
        base = base.wrapping_mul(base) % modulus;
        exp /= 2;
    }
    result
}
