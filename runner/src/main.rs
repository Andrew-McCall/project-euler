use std::fmt::Display;
use std::time::Instant;

static mut RAN: bool = false;

#[cfg(any(
    all(feature = "bench3", feature = "bench10"),
    all(feature = "bench3", feature = "bench100"),
    all(feature = "bench3", feature = "bench1000"),
    all(feature = "bench10", feature = "bench100"),
    all(feature = "bench10", feature = "bench1000"),
    all(feature = "bench100", feature = "bench1000"),
))]
compile_error!("Only one benchmarking feature may be enabled at a time.");

fn benchmark<T: Display>(action: impl Fn() -> T) {
    unsafe { RAN = true };

    #[cfg(not(feature = "skipwarm"))]
    let _ = action();

    const RUNS: u32 = if cfg!(feature = "bench3") {
        3
    } else if cfg!(feature = "bench10") {
        10
    } else if cfg!(feature = "bench100") {
        100
    } else if cfg!(feature = "bench1000") {
        1000
    } else {
        1
    };

    let start = Instant::now();
    let result = action();
    for _ in 1..RUNS {
        std::hint::black_box(action());
    }
    let duration = start.elapsed() / RUNS;

    println!("Answer: {}", result);
    println!("Time elapsed: {:.8} seconds\n", duration.as_secs_f64());
}

fn main() {
    #[cfg(feature = "s1")]
    {
        println!("Solving Problem 1:");
        benchmark(|| p1::solution(1_000));
    }

    #[cfg(feature = "s2")]
    {
        println!("Solving Problem 2:");
        benchmark(|| p2::solution(4_000_000));
    }

    #[cfg(feature = "s3")]
    {
        println!("Solving Problem 3:");
        benchmark(|| p3::solution(600_851_475_143));
    }

    #[cfg(feature = "s4")]
    {
        println!("Solving Problem 4:");
        benchmark(|| p4::solution(999));
    }

    #[cfg(feature = "s5")]
    {
        println!("Solving Problem 5:");
        benchmark(|| p5::solution(20));
    }

    #[cfg(feature = "s6")]
    {
        println!("Solving Problem 6:");
        benchmark(|| p6::solution(100));
        println!("Solving Problem 6 alt:");
        benchmark(|| p6::alt_solution(100));
    }

    #[cfg(feature = "s7")]
    {
        println!("Solving Problem 7:");
        benchmark(|| p7::solution(10_001));
    }

    #[cfg(feature = "s8")]
    {
        println!("Solving Problem 8:");
        benchmark(|| p8::solution(p8::string_to_array(p8::INPUT), 13));
    }

    #[cfg(feature = "s9")]
    {
        println!("Solving Problem 9:");
        benchmark(p9::solution)
    }

    #[cfg(feature = "s10")]
    {
        println!("Solving Problem 10:");
        benchmark(|| p10::solution(2_000_000));
    }

    #[cfg(feature = "s11")]
    {
        println!("Solving Problem 11:");
        benchmark(|| p11::solution(p11::spaced_string_to_array(p11::INPUT), 20, 20, 4));
    }

    #[cfg(feature = "s12")]
    {
        println!("Solving Problem 12:");
        benchmark(|| p12::solution(500));
    }

    #[cfg(feature = "s13")]
    {
        println!("Solving Problem 13:");
        benchmark(|| p13::solution(p13::INPUT.split_whitespace().collect::<Vec<_>>(), 10));
    }

    #[cfg(feature = "s14")]
    {
        println!("Solving Problem 14:");
        benchmark(|| p14::solution(1_000_000));
    }

    #[cfg(feature = "s15")]
    {
        println!("Solving Problem 15:");
        benchmark(|| p15::solution(20));
    }

    #[cfg(feature = "s15")]
    {
        println!("Solving Problem 15:");
        benchmark(|| p15::solution(20));
    }

    #[cfg(feature = "s16")]
    {
        println!("Solving Problem 16:");
        benchmark(|| p16::solution(2, 1_000));
    }

    #[cfg(feature = "s17")]
    {
        println!("Solving Problem 17:");
        benchmark(|| p17::solution(1_000));
    }

    #[cfg(feature = "s18")]
    {
        println!("Solving Problem 18:");
        benchmark(|| p18::solution(p18::string_to_triangle(p18::INPUT)));
    }

    #[cfg(feature = "s19")]
    {
        println!("Solving Problem 19:");
        benchmark(|| {
            p19::solution(
                p19::Date {
                    year: 1901,
                    month: p19::Month::January,
                    day: 1,
                    weekday: p19::DayOfWeek::Tuesday,
                },
                p19::Date {
                    year: 2000,
                    month: p19::Month::December,
                    day: 31,
                    weekday: p19::DayOfWeek::Sunday,
                },
            )
        });
    }

    #[cfg(feature = "s20")]
    {
        println!("Solving Problem 20:");
        benchmark(|| p20::solution(100));
    }

    #[cfg(feature = "s21")]
    {
        println!("Solving Problem 21:");
        benchmark(|| p21::solution(10_000));
    }

    #[cfg(feature = "s22")]
    {
        println!("Solving Problem 22:");
        benchmark(|| p22::solution(p22::file_to_names(&format!("p022/{}", p22::INPUT_PATH))));
    }

    #[cfg(feature = "s23")]
    {
        println!("Solving Problem 23:");
        benchmark(p23::solution);
    }

    #[cfg(feature = "s24")]
    {
        println!("Solving Problem 24:");
        benchmark(|| p24::solution(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0], 1_000_000));
    }

    #[cfg(feature = "s25")]
    {
        println!("Solving Problem 25:");
        benchmark(|| p25::solution(1_000));
    }

    #[cfg(feature = "s26")]
    {
        println!("Solving Problem 26:");
        benchmark(|| p26::solution(1_000));
    }

    #[cfg(feature = "s27")]
    {
        println!("Solving Problem 27:");
        benchmark(|| p27::solution(1_000));
    }

    #[cfg(feature = "s28")]
    {
        println!("Solving Problem 28:");
        benchmark(|| p28::solution(1_001));
    }

    #[cfg(feature = "s29")]
    {
        println!("Solving Problem 29:");
        benchmark(|| p29::solution(100));
    }

    #[cfg(feature = "s30")]
    {
        println!("Solving Problem 30:");
        benchmark(|| p30::solution(5));
    }

    #[cfg(feature = "s31")]
    {
        println!("Solving Problem 31:");
        benchmark(|| p31::solution(&[1, 5, 2, 10, 20, 50, 100, 200], 200));
    }

    #[cfg(feature = "s32")]
    {
        println!("Solving Problem 32:");
        benchmark(p32::solution);
    }

    #[cfg(feature = "s33")]
    {
        println!("Solving Problem 33:");
        benchmark(p33::solution);
    }

    #[cfg(feature = "s34")]
    {
        println!("Solving Problem 34:");
        benchmark(p34::solution);
    }

    #[cfg(feature = "s35")]
    {
        println!("Solving Problem 35:");
        benchmark(|| p35::solution(1_000_000));
    }

    #[cfg(feature = "s36")]
    {
        println!("Solving Problem 36:");
        benchmark(|| p36::solution(1_000_000));
    }

    #[cfg(feature = "s37")]
    {
        println!("Solving Problem 37:");
        benchmark(p37::solution);
    }

    #[cfg(feature = "s38")]
    {
        println!("Solving Problem 38:");
        benchmark(p38::solution);
    }

    #[cfg(feature = "s39")]
    {
        println!("Solving Problem 39:");
        benchmark(|| p39::solution(1_000));
    }

    #[cfg(feature = "s40")]
    {
        println!("Solving Problem 40:");
        benchmark(|| p40::solution(1_000_000));
    }

    #[cfg(feature = "s41")]
    {
        println!("Solving Problem 41:");
        benchmark(p41::solution);
    }

    #[cfg(feature = "s42")]
    {
        println!("Solving Problem 42:");
        benchmark(|| p42::solution(p42::file_to_words(&format!("p042/{}", p42::EXAMPLE_FILE))));
    }

    #[cfg(feature = "s43")]
    {
        println!("Solving Problem 43:");
        benchmark(p43::solution);
    }

    #[cfg(feature = "s44")]
    {
        println!("Solving Problem 44:");
        benchmark(|| p44::solution(10_000));
    }

    #[cfg(feature = "s45")]
    {
        println!("Solving Problem 45:");
        benchmark(|| p45::solution(286, 100_000));
    }

    #[cfg(feature = "s46")]
    {
        println!("Solving Problem 46:");
        benchmark(p46::solution);
    }

    #[cfg(feature = "p52")]
    {
        println!("Solving Problem 52:");
        benchmark(|| p52::solution(6));
    }

    #[cfg(feature = "s67")]
    {
        println!("Solving Problem 67:");
        benchmark(|| p67::solution(&format!("p067/{}", p67::INPUT_PATH)));
    }

    if !unsafe { RAN } {
        println!("No solution(s) selected:");
    }
}
