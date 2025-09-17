/* p27: Quadratic Primes */
pub fn solution(bound: i64) -> Result {
    let mut best = (39, 1, 41);
    for a in (-bound)..bound {
        for b in (-bound)..bound {
            let mut n = 0;
            while is_prime(n * n + n * a + b) {
                n += 1;
            }
            if n > best.0 {
                best.0 = n;
                best.1 = a;
                best.2 = b;
            }
        }
    }

    Result {
        l: best.0 as u64,
        a: best.1,
        b: best.2,
    }
}

#[test]
fn test() {
    let res = solution(100);
    println!("{}", res); // -1455 (48) <= n^2 + n(-15) + 97
    assert_eq!(res.get_product(), -1455);

    let res = solution(1_000);
    println!("{}", res); // -59231 (71) <= n^2 + n(-61) + 971

    assert_eq!(res.get_product(), -59231);

    let res = solution(10_000);
    println!("{}", res); // -126479 (80) <= n^2 + n(-79) + 1601
    assert_eq!(res.get_product(), -126479);
}

fn is_prime(n: i64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 || n == 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

#[derive(Debug, PartialEq, Eq)]
pub struct Result {
    l: u64,
    a: i64,
    b: i64,
}

impl std::fmt::Display for Result {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({}) <= n^2 + n({}) + {}",
            self.get_product(),
            self.l,
            self.a,
            self.b
        )
    }
}

impl Result {
    fn get_product(&self) -> i64 {
        self.a * self.b
    }
}
