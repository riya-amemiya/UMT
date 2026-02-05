use std::time::Instant;
use umt_rust::math::umt_uuidv7;

#[test]
fn benchmark_uuid() {
    let iterations = 100_000;

    let start = Instant::now();
    for _ in 0..iterations {
        let _ = umt_uuidv7();
    }
    let duration = start.elapsed();

    println!("UUID Benchmark: {:?}", duration);
}
