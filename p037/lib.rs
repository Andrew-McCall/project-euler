/* p37: Truncatable Primes */
pub fn solution() -> u64 {
    let mut sum = 0;

    for n in (11..1_000_000).step_by(2) {
        if !is_prime(n) {
            continue;
        }

        let mut left = n;
        let mut right = n;

        let mut all_prime = true;
        while left > 10 {
            left = trunc_left(left);

            if !is_prime(left) {
                all_prime = false;
                break;
            }
        }

        if all_prime {
            while right > 10 {
                right = trunc_right(right);

                if !is_prime(right) {
                    all_prime = false;
                    break;
                }
            }
        }

        if all_prime {
            sum += n;
        }
    }
    sum
}

#[test]
fn test() {
    assert_eq!(solution(), 0);
}

fn trunc_left(n: u64) -> u64 {
    n / 10
}

fn trunc_right(n: u64) -> u64 {
    let mask = 10_u64.pow((n as f32).log10() as u32);

    n % mask
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
