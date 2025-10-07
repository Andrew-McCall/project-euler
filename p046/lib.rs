/* p46: Goldbach's Other Conjecture */
pub fn solution() -> u64 {
    let mut primes = vec![2];
    let mut comp = 3;

    'outer: loop {
        if is_prime(comp) {
            primes.push(comp);
        } else {
            for &p in primes.iter() {
                if p >= comp {
                    break;
                }
                let remainder = comp - p;
                if is_perfect_square(remainder / 2) {
                    comp += 2;
                    continue 'outer;
                }
            }
            return comp;
        }

        comp += 2;
    }
}

#[test]
fn test() {
    assert_eq!(solution(), 0);
}

fn is_perfect_square(n: u64) -> bool {
    if n < 2 {
        return true;
    }
    let mut left = 1;
    let mut right = n / 2;
    while left <= right {
        let mid = left + (right - left) / 2;
        if mid <= n / mid {
            if mid * mid == n {
                return true;
            }
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    false
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

    let mut i = 3;
    while i * i <= n {
        if n.is_multiple_of(i) {
            return false;
        }
        i += 2;
    }
    true
}
