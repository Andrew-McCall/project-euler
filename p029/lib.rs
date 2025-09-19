/* p29: Distinct Powers */
pub fn solution(max_base: u64) -> u64 {
    let mut distinct = std::collections::HashSet::new();

    for a in 2..=max_base {
        for b in 2..=max_base {
            let mut base = a;
            let exponent = b;

            let mut factors = Vec::new();

            let mut i = 2;
            while i * i <= base {
                let mut count = 0;
                while base % i == 0 {
                    base /= i;
                    count += 1;
                }
                if count > 0 {
                    factors.push((i, (count as u64) * exponent));
                }
                if i == 2 {
                    i = 3;
                } else {
                    i += 2;
                }
            }

            if base > 1 {
                factors.push((base, exponent));
            }

            let exponent_gcd = factors
                .iter()
                .map(|e| e.1)
                .fold(0, |acc, x| if acc == 0 { x } else { gcd(acc, x) });

            let smallest_base = if exponent_gcd == 0 {
                a
            } else {
                factors
                    .iter()
                    .flat_map(|&(p, exp_after)| {
                        std::iter::repeat_n(p, (exp_after / exponent_gcd) as usize)
                    })
                    .product::<u64>()
            };

            distinct.insert((exponent_gcd, smallest_base));
        }
    }

    distinct.len() as u64
}

#[test]
fn test() {
    assert_eq!(solution(5), 15);
    assert_eq!(solution(100), 9183);
}

// Greatest Common Divisor
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}
