/* p43: Sub-string Divisibility */
pub fn solution() -> u64 {
    let mut seq = [9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
    let original = seq;
    seq.sort();
    let mut sum = 0;
    let len = seq.len();

    while seq != original {
        let mut index = len - 1;
        while index > 0 && seq[index - 1] >= seq[index] {
            index -= 1;
        }
        if index == 0 {
            break;
        }

        let pivot = index - 1;

        let mut swap = len - 1;
        while seq[swap] <= seq[pivot] {
            swap -= 1;
        }

        seq.swap(pivot, swap);
        seq[pivot + 1..].reverse();

        let mut slice = concat(seq[1], seq[2], seq[3]);

        if !slice.is_multiple_of(2) {
            continue;
        }
        slice = concat_next(slice, seq[4]);
        if !slice.is_multiple_of(3) {
            continue;
        }
        slice = concat_next(slice, seq[5]);
        if !slice.is_multiple_of(5) {
            continue;
        }
        slice = concat_next(slice, seq[6]);
        if !slice.is_multiple_of(7) {
            continue;
        }
        slice = concat_next(slice, seq[7]);
        if !slice.is_multiple_of(11) {
            continue;
        }
        slice = concat_next(slice, seq[8]);
        if !slice.is_multiple_of(13) {
            continue;
        }
        slice = concat_next(slice, seq[9]);
        if !slice.is_multiple_of(17) {
            continue;
        }

        sum += seq.iter().fold(0, |acc, &d| acc * 10 + d);
    }

    sum
}

#[test]
fn test() {
    assert_eq!(concat(1, 2, 3), 123);
    assert_eq!(concat(9, 9, 9), 999);
    assert_eq!(concat_next(999, 1), 991);

    assert_eq!(solution(), 0);
}

fn concat(first: u64, second: u64, third: u64) -> u64 {
    let mut res = first;
    res *= 10;
    res += second;

    res *= 10;
    res += third;

    res
}

fn concat_next(mut base: u64, next: u64) -> u64 {
    base *= 10;
    base += next;

    base % 1000
}
