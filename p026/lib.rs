/* p26: Reciprocal Cycles */
pub fn solution(bound: u64) -> u64 {
    let accuracy = bound * 2;

    let mut longest: (u64, u64) = (1, 0);

    for i in 2..bound {
        let mut current_digit: u64;
        let mut digits = Vec::new();

        let mut remainder = 1;
        let mut divide;

        while remainder > 0 {
            current_digit = 10 * remainder;

            if digits.len() as u64 >= accuracy {
                break;
            }
            remainder = current_digit % i;
            divide = current_digit / i;
            digits.push(divide);
        }

        if digits.len() as u64 != accuracy {
            continue;
        }

        for r in 1..(bound - 10) {
            // Can't confirm repeating if larger than half digits length (accuracy is bound * 2)
            let ru = r as usize;
            if digits[10..10 + ru] == digits[10 + ru..10 + (ru * 2)] {
                if r > longest.0 {
                    longest.0 = r;
                    longest.1 = i;
                }
                break;
            }
        }
    }

    longest.1
}

#[test]
fn test() {
    assert_eq!(solution(20), 19);
    assert_eq!(solution(1000), 983);
}
