use std::collections::HashSet;

/* p32: Pandigital Products */
pub fn solution() -> u64 {
    let mut set = HashSet::new();

    let bound = 10_u64.pow(4);

    for a in 1..bound {
        for b in (a + 1)..bound {
            let mut used = [0; 10];

            mark_digit(a, &mut used);
            mark_digit(b, &mut used);

            let p = a * b;

            mark_digit(p, &mut used);

            if used[0] == 0
                && used[1] == 1
                && used[2] == 1
                && used[3] == 1
                && used[4] == 1
                && used[5] == 1
                && used[6] == 1
                && used[7] == 1
                && used[8] == 1
                && used[9] == 1
            {
                set.insert(p);
            }
        }
    }

    set.into_iter().sum()
}

fn mark_digit(mut n: u64, marks: &mut [u8; 10]) {
    while n > 0 {
        marks[(n % 10) as usize] += 1;
        n /= 10;
    }
}

#[test]
fn test() {
    assert_eq!(solution(), 45228);
}
