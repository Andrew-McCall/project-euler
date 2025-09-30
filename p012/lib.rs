/* p12: Highly Divisible Triangular Number */
pub fn solution(n: u64) -> u64 {
    let mut current = 1;
    let mut index = 1;

    while count_factors(current) < n {
        index += 1;
        current += index;
    }

    current
}

#[test]
fn test() {
    assert_eq!(solution(4), 6);
    assert_eq!(solution(500), 76576500);
}

#[test]
fn test_count_factors() {
    assert_eq!(count_factors(3), 2);
    assert_eq!(count_factors(6), 4);
    assert_eq!(count_factors(10), 4);
    assert_eq!(count_factors(15), 4);
}

fn count_factors(n: u64) -> u64 {
    let sqrt = (n as f64).sqrt() as u64;
    let mut count = 0;

    for i in 1..=sqrt {
        if n.is_multiple_of(i) {
            count += 2;
        }
    }

    if sqrt * sqrt == n {
        count -= 1;
    }

    count
}
