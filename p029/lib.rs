/* p29: Distinct Powers */
pub fn solution(max_base: u64) -> u64 {
    let mut base = max_base;
    let exponent = max_base;

    // Prime Factorization
    let mut factors_base = Vec::new();

    let mut i = 2;

    while i * i < (base + 1) {
        let mut count = 0;
        while base % i == 0 {
            base /= i;
            count += 1;

            if base < i {
                break;
            }
        }
        if count > 0 {
            factors.push((i, count * exponent));
        }
        if i == 2 {
            i = 3
        } else {
            i += 2;
        }
    }

    if base > 0 {
        factors.push((base, exponent));
    }

    println!("{:?}", factors);

    0
}

#[test]
fn test() {
    solution(5);
    solution(100);
    solution(1234567890);
    // assert_eq!(solution(5), 15);
    // assert_eq!(solution(100), 9183);
    // assert_eq!(solution(1000), 9183);
}

// Greatest Common Divisor thing
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}
