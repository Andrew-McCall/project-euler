pub fn solution(limit: usize) -> usize {
    let mut sum = 0;
    for i in 1..limit {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }
    sum
}

// TODO: Lookup Rust magic numbers/branchless solution
