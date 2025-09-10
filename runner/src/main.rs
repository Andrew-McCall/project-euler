use std::fmt::Display;
use std::time::Instant;

static mut RAN: bool = false;

fn benchmark<T: Display>(action: impl FnOnce() -> T) {
    unsafe { RAN = true };

    let start = Instant::now();
    let result = action();
    let duration = start.elapsed();

    println!("{}", result);
    println!("Time elapsed: {:.8} seconds", duration.as_secs_f64());
}

fn main() {
    #[cfg(feature = "s1")]
    {
        println!("Running p1");
        benchmark(|| p1::solution(1000));
    }

    #[cfg(feature = "s2")]
    {
        println!("Running p2");
        benchmark(|| p2::solution(4000000));
    }

    #[cfg(feature = "s3")]
    {
        println!("Running p3");
        benchmark(|| p3::solution(600851475143));
    }

    #[cfg(feature = "s4")]
    {
        println!("Running p4");
        benchmark(|| p4::solution(999));
    }

    #[cfg(feature = "s5")]
    {
        println!("Running p5");
        benchmark(|| p5::solution(20));
    }

    #[cfg(feature = "s6")]
    {
        println!("Running p6");
        benchmark(|| p6::solution(100));
        println!("Running p6 alt");
        benchmark(|| p6::alt_solution(100));
    }

    #[cfg(feature = "s7")]
    {
        println!("Running p7");
        benchmark(|| p7::solution(10001));
    }

    if !unsafe { RAN } {
        println!("No solution(s) selected");
    }
}
