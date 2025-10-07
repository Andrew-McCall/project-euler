/* p44: Pentagon Numbers */
pub fn solution(bound: u64) -> u64 {
    let seq: Vec<u64> = (1..=bound).map(|n| n * (n * 3 - 1) / 2).collect();
    let bound = bound as usize;

    for i in 0..bound {
        for j in (i + 1)..bound {
            let i = seq[i];
            let j = seq[j];
            let sum = i + j;
            let delta = j - i;

            if seq.binary_search(&sum).is_err() {
                continue;
            }

            if seq.binary_search(&delta).is_err() {
                continue;
            }

            return delta;
        }
    }
    0
}

#[test]
fn test() {
    assert_eq!(solution(10_000), 5482660);
}
