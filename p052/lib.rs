/* p52: Permuted Multiples */
pub fn solution(permutation: u64) -> u64 {
    let mut x = 1;

    'outer: loop {
        let digits = get_digits(x);
        for p in 2..=permutation {
            if get_digits(p * x) != digits {
                x += 1;
                continue 'outer;
            }
        }

        // Added exactly check so solution(2..5) would not equal s(6)
        if get_digits(x * (permutation + 1)) == digits {
            x += 1;
            continue 'outer;
        }

        return x;
    }
}

#[test]
fn test() {
    assert_eq!(solution(2), 125874);
    assert_eq!(solution(6), 142857);
}

#[test]
fn other() {
    assert_eq!(solution(3), 285714);
    assert_eq!(solution(4), 0);
}

fn get_digits(mut n: u64) -> [u8; 10] {
    let mut digits = [0; 10];

    while n > 0 {
        digits[(n % 10) as usize] += 1;
        n /= 10;
    }

    digits
}
