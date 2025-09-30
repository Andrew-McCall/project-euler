/* p35: Circular Prime */
pub fn solution(bound: u64) -> u64 {
    let mut found = Vec::new();

    for mut x in 3..bound {
        if found.contains(&x) {
            continue;
        }

        let length = (x as f64).log10() as u32 + 1;
        let base = 10_u64.pow(length - 1);
        let mut current = Vec::new();

        let mut all_prime = true;
        for _ in 0..length {
            if !is_prime(x) {
                all_prime = false;
                break;
            };

            current.push(x);
            x = rotate_number(x, base);
        }
        if all_prime {
            found.append(&mut current);
        }
    }

    found.len() as u64
}

fn rotate_number(mut n: u64, base: u64) -> u64 {
    let last = n % 10;
    n /= 10;

    n + last * base
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

#[test]
fn test() {
    assert_eq!(solution(100), 13);
    assert_eq!(solution(1_000_000), 55);
    assert_eq!(solution(10_000_000), 55);
}
