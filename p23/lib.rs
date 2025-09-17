/* p23: Non-Abundant Sums */
pub fn solution() -> u64 {
    let mut abundant_numbers = vec![12];

    for i in 13..BOUND {
        if is_abundant(i) {
            abundant_numbers.push(i);
        }
    }

    let mut combinations: Vec<u64> = (1..=BOUND).collect();

    for i in &abundant_numbers {
        for j in &abundant_numbers {
            let comb = i + j;
            if comb <= BOUND {
                combinations[(comb - 1) as usize] = 0;
            }
        }
    }

    combinations.iter().sum()
}

const BOUND: u64 = 28123;

#[test]
fn test() {
    assert_eq!(solution(), 0);
}

fn is_abundant(n: u64) -> bool {
    let mut sum = 1;
    for d in 2..n {
        if n % d == 0 {
            sum += d;
        }
    }
    sum > n
}
