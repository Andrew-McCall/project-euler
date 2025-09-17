/* p21: Amicable Numbers */
pub fn solution(bound: u64) -> u64 {
    assert!(bound <= usize::MAX as u64);

    let mut result = 0;
    let mut divsor_sums = vec![0; bound as usize];

    for i in 0..(bound - 1) {
        if divsor_sums[i as usize] > 0 {
            continue;
        }

        let sum = get_proper_divisors_sum(i);
        divsor_sums[i as usize] = sum;
        if sum <= bound && divsor_sums[sum as usize] == 0 {
            let other_sum = get_proper_divisors_sum(sum);
            divsor_sums[sum as usize] = other_sum;

            if other_sum == i {
                result += i;
                result += sum;
            }
        }
    }

    result
}

#[test]
fn test() {
    assert_eq!(solution(10_000), 31626);
    // assert_eq!(solution(100_000), 852810);
}

pub fn get_proper_divisors_sum(n: u64) -> u64 {
    let mut sum = 1;

    for d in 2..n {
        if n % d == 0 {
            sum += d;
        }
    }

    sum
}

#[test]
fn test_get_proper_divisors_sum() {
    assert_eq!(get_proper_divisors_sum(220), 284);
    assert_eq!(get_proper_divisors_sum(284), 220);
}
