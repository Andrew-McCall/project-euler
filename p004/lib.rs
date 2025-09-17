/* p4: Largest Palindrome Product */
pub fn solution(bound: u64) -> Result {
    let mut largest = Result {
        product: 0,
        factors: (0, 0),
    };
    for x in 0..bound {
        for y in x..bound {
            let product = (bound - x) * (bound - y);
            if product > largest.product && is_palindromic(product) {
                largest = Result {
                    product,
                    factors: (bound - x, bound - y),
                }
            }
        }
    }
    largest
}

#[test]
fn test() {
    assert_eq!(solution(100).product, 9009);
    assert_eq!(solution(999).product, 906609);
    assert_eq!(solution(9999).product, 99000099);
}

#[derive(Debug, Eq, PartialEq)]
pub struct Result {
    pub product: u64,
    pub factors: (u64, u64),
}

impl std::fmt::Display for Result {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} = {} * {}",
            self.product, self.factors.0, self.factors.1
        )
    }
}

fn is_palindromic(mut n: u64) -> bool {
    let original = n;
    let mut reversed = 0;

    while n > 0 {
        reversed = reversed * 10 + (n % 10);
        n /= 10;
    }

    original == reversed
}
