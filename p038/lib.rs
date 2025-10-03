/* p38: Pandigital Multiples */
pub fn solution() -> u64 {
    let mut largest = 0;
    for x in (1..99999).rev() {
        let mut digits = Vec::new();
        let mut n = 0;
        while digits.len() < 9 {
            let mut prod = x * n;
            let mut stack = Vec::new();

            while prod > 0 {
                stack.push((prod % 10) as u8);
                prod /= 10;
            }

            while let Some(d) = stack.pop() {
                digits.push(d);
            }

            n += 1;
        }

        if validate(&digits) {
            let current = digits.iter().fold(0_u64, |acc, &d| acc * 10 + d as u64);
            if current > largest {
                largest = current;
            }
        }
    }

    largest
}

#[test]
fn test() {
    assert_eq!(solution(), 932718654);
}

fn validate(digits: &[u8]) -> bool {
    if digits.len() < 9 {
        return false;
    }

    let mut used = [0_u8; 10];

    for &d in digits {
        used[d as usize] += 1;
    }

    used[0] == 0
        && used[1] == 1
        && used[2] == 1
        && used[3] == 1
        && used[4] == 1
        && used[5] == 1
        && used[6] == 1
        && used[7] == 1
        && used[8] == 1
        && used[9] == 1
}
