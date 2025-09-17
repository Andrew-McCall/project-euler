/* p14: Longest Collatz Sequence */
pub fn solution(bound: u64) -> Result {
    let mut longest_length: u64 = 2;
    let mut longest: u64 = 2;

    for i in 3..bound {
        let mut current = i;
        let mut length = 0;
        while current > 1 {
            length += 1;

            if current & 1 == 0 {
                current /= 2
            } else {
                current = current * 3 + 1
            }
        }

        if length > longest_length {
            longest_length = length;
            longest = i;
        }
    }

    Result {
        length: longest_length,
        start: longest,
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Result {
    pub length: u64,
    pub start: u64,
}

impl std::fmt::Display for Result {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.start, self.length)
    }
}

#[test]
fn test() {
    assert_eq!(
        solution(10),
        Result {
            length: 19,
            start: 9
        }
    );

    assert_eq!(solution(1_000_000).start, 837799);
}
