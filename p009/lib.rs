/* p9: Special Pythagorean Triplet */
pub fn solution() -> Result {
    for a in 1..999 {
        for b in a + 1..999 {
            for c in b + 1..999 {
                if a + b + c == 1000 && a * a + b * b == c * c {
                    return Result { a, b, c };
                }
            }
        }
    }
    panic!("Failed to find the Special Pythagorean triplet");
}

#[test]
fn test() {
    let res = solution();
    assert_eq!(res.a, 200);
    assert_eq!(res.b, 375);
    assert_eq!(res.c, 425);
    assert_eq!(res.product(), 31875000);
}

pub struct Result {
    a: u64,
    b: u64,
    c: u64,
}

impl std::fmt::Display for Result {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} = {} * {} * {}",
            self.product(),
            self.a,
            self.b,
            self.c
        )
    }
}

impl Result {
    pub fn product(&self) -> u64 {
        self.a * self.b * self.c
    }
}
