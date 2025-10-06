/* p24: Lexicographic Permutations */
pub fn solution(mut seq: Vec<u64>, n: u64) -> u64 {
    seq.sort();

    for _ in 1..n {
        let len = seq.len();

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
    }

    seq.iter().fold(0, |acc, &d| acc * 10 + d)
}

#[test]
fn test() {
    assert_eq!(solution(vec![1, 2, 3, 0], 1), 123);
    assert_eq!(solution(vec![1, 2, 3, 0], 2), 132);
    assert_eq!(
        solution(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0], 1_000_000),
        2783915460
    );
}
