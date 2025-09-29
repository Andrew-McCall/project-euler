/* p33: Digit Factorials */
const BOUND: u64 = 10_u64.pow(5);

pub fn solution() -> u64 {
    let mut result = 0;

    for n in 3..BOUND {
        let mut current = n;
        let mut digit_sum = 0;
        while current > 0 && digit_sum < n {
            digit_sum += factorial(current % 10);
            current /= 10;
        }

        if current == 0 && digit_sum == n {
            result += n;
        }
    }

    result
}

fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

#[test]
fn test() {
    assert_eq!(solution(), 0);
}
