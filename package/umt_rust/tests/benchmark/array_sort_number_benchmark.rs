use std::time::Instant;
use umt_rust::simple::array::umt_quick_sort_simple_f64;
use rand::Rng;

#[test]
fn benchmark_sort_number() {
    let size = 100_000;
    let mut rng = rand::rng();
    let input: Vec<f64> = (0..size).map(|_| rng.random()).collect();

    let start = Instant::now();
    let _result = umt_quick_sort_simple_f64(&input, true, None, None);
    let duration = start.elapsed();

    println!("Sort Number Benchmark: {:?}", duration);
}
