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
        benchmark(|| {
            p8::solution(
                p8::string_to_array(
                    "7316717653133062491922511967442657474235534919493496983520312774506326239578318016984801869478851843858615607891129494954595017379583319528532088055111254069874715852386305071569329096329522744304355766896648950445244523161731856403098711121722383113622298934233803081353362766142828064444866452387493035890729629049156044077239071381051585930796086670172427121883998797908792274921901699720888093776657273330010533678812202354218097512545405947522435258490771167055601360483958644670632441572215539753697817977846174064955149290862569321978468622482839722413756570560574902614079729686524145351004748216637048440319989000889524345065854122758866688116427171479924442928230863465674813919123162824586178664583591245665294765456828489128831426076900422421902267105562632111110937054421750694165896040807198403850962455444362981230987879927244284909188845801561660979191338754992005240636899125607176060588611646710940507754100225698315520005593572972571636269561882670428252483600823257530420752963450",
                ),
                13,
            )
        });
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

    if !unsafe { RAN } {
        println!("No solution(s) selected:");
    }
}
