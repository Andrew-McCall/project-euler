/* p30: Digit Fifth Powers */
pub fn solution(l: u32) -> u64 {
    let mut result = 0;
    for i in 2..10_u64.pow(l + l / 2) {
        let mut current = i;
        let mut digit_sum = 0;
        while current > 0 {
            let digit = current % 10;
            current /= 10;
            digit_sum += digit.pow(l);
        }
        if digit_sum == i {
            result += i;
        }
    }

    result
}

#[test]
fn test() {
    assert_eq!(solution(4), 19316);
    assert_eq!(solution(5), 443839);
}
