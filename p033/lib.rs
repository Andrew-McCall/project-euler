/* p33: Digit Cancelling Fractions */
pub fn solution() -> u64 {
    let mut num_prod = 1;
    let mut den_prod = 1;

    for numerator in 10..100 {
        for denominator in (numerator + 1)..100 {
            let n1 = numerator / 10;
            let n2 = numerator % 10;
            let d1 = denominator / 10;
            let d2 = denominator % 10;

            if n2 == 0 && d2 == 0 {
                continue;
            }

            if (n1 == d1 && d2 != 0 && numerator * d2 == n2 * denominator)
                || (n1 == d2 && d1 != 0 && numerator * d1 == n2 * denominator)
                || (n2 == d1 && d2 != 0 && numerator * d2 == n1 * denominator)
                || (n2 == d2 && d1 != 0 && numerator * d1 == n1 * denominator)
            {
                num_prod *= numerator;
                den_prod *= denominator;
            }
        }
    }

    den_prod / gcd(num_prod, den_prod)
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

#[test]
fn test() {
    assert_eq!(solution(), 100);
}
