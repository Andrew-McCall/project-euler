/* p40: Champernowne's Constant */
pub fn solution(bound: u64) -> u64 {
    let mut product = 1;
    let mut number = 1;
    let mut index = 1_u64;

    fn is_power_of_10(n: u64) -> bool {
        if n == 0 {
            return false;
        }
        let mut n = n;
        while n % 10 == 0 {
            n /= 10;
        }
        n == 1
    }

    while index <= bound {
        let mut digits = number;
        let mut digits_vec = Vec::new();

        while digits > 0 {
            digits_vec.push(digits % 10);
            digits /= 10;
        }
        digits_vec.reverse();

        for &d in &digits_vec {
            if is_power_of_10(index) {
                product *= d as u64;
            }
            index += 1;
        }

        number += 1;
    }

    product
}

#[test]
fn test() {
    assert_eq!(solution(1_000_000), 210);
}
