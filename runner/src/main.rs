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
        benchmark(p2::solution);
    }

    if !unsafe { RAN } {
        println!("No solution(s) selected");
    }
}
