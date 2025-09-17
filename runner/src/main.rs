use std::fmt::Display;
use std::time::Instant;

static mut RAN: bool = false;

fn benchmark<T: Display>(action: impl FnOnce() -> T) {
    unsafe { RAN = true };

    let start = Instant::now();
    let result = action();
    let duration = start.elapsed();

    println!("{}", result);
    println!("Time elapsed: {:.8} seconds\n", duration.as_secs_f64());
}

fn main() {
    #[cfg(feature = "s1")]
    {
        println!("Solving Problem 1:");
        benchmark(|| p1::solution(1000));
    }

    #[cfg(feature = "s2")]
    {
        println!("Solving Problem 2:");
        benchmark(|| p2::solution(4000000));
    }

    #[cfg(feature = "s3")]
    {
        println!("Solving Problem 3:");
        benchmark(|| p3::solution(600851475143));
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
        benchmark(|| p7::solution(10001));
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
        benchmark(|| p10::solution(2000000));
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
        benchmark(|| p16::solution(2, 1000));
    }

    #[cfg(feature = "s17")]
    {
        println!("Solving Problem 17:");
        benchmark(|| p17::solution(1000));
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
                Date {
                    year: 1901,
                    month: Month::January,
                    day: 1,
                    weekday: DayOfWeek::Tuesday,
                },
                Date {
                    year: 2000,
                    month: Month::December,
                    day: 31,
                    weekday: DayOfWeek::Sunday,
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
        benchmark(|| p22::solution(p22::INPUT_PATH));
    }

    #[cfg(feature = "s67")]
    {
        println!("Solving Problem 67:");
        benchmark(|| p67::solution(p67::INPUT_PATH));
    }

    if !unsafe { RAN } {
        println!("No solution(s) selected:");
    }
}
