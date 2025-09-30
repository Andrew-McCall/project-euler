/* p39: Integer Right Triangles */
pub fn solution(bound: u64) -> u64 {
    let mut counts = vec![0_u32; (bound + 1) as usize];

    for a in 1..=bound / 2 {
        for b in a..=bound / 2 {
            let c2 = a * a + b * b;
            let c = (c2 as f64).sqrt() as u64;
            if c * c == c2 {
                let p = a + b + c;
                if p <= bound {
                    counts[p as usize] += 1;
                }
            }
        }
    }

    let mut best_p = 0;
    let mut best_count = 0;

    for (p, &count) in counts.iter().enumerate() {
        if count > best_count {
            best_count = count;
            best_p = p as u64;
        }
    }

    best_p
}

#[test]
fn test() {
    assert_eq!(solution(1000), 840);
    assert_eq!(solution(10_000), 9240);
}
