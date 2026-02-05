use std::time::Instant;
use umt_rust::array::umt_group_by;

#[test]
fn benchmark_group_by_math_floor() {
    let size = 100_000;
    let input: Vec<f64> = (0..size).map(|i| i as f64 / 10.0).collect();

    let start = Instant::now();
    // In Rust we pass a closure directly
    let _result = umt_group_by(&input, |n| n.floor().to_string());
    let duration = start.elapsed();

    println!("Group By Math Floor Benchmark: {:?}", duration);
}
