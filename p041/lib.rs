/* p32: Pandigital Prime */
pub fn solution() -> u64 {
    for p in (1..7_654_321).rev() {
        if is_pandigital(p) && is_prime(p) {
            return p;
        }
    }

    0
}

#[test]
fn test() {
    assert_eq!(solution(), 7652413);
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

    for i in (3..=(n as f64).sqrt() as u64).step_by(2) {
        if n.is_multiple_of(i) {
            return false;
        }
    }
    true
}

fn is_pandigital(mut n: u64) -> bool {
    if n > 987_654_321 {
        return false;
    }
    let mut count = 0;

    let mut digits = [0; 10];

    while n > 0 {
        let d = (n % 10) as usize;
        if d == 0 || digits[d] > 0 {
            return false;
        }
        digits[d] += 1;
        n /= 10;
        count += 1;
    }

    (1..=count).all(|i| digits[i] == 1)
}
