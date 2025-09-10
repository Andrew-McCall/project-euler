use std::time::Instant;

fn main() {
    #[cfg(feature = "")]
    #[cfg(feature = "p1")]
    {
        let start = Instant::now();
        let result = p1::solution(1000);
        let duration = start.elapsed();

        println!("p1: {}", result);
        println!("Time elapsed: {:.8} seconds", duration.as_secs_f64());
    }
}
