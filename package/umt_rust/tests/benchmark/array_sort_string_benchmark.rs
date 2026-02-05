use std::time::Instant;
use umt_rust::simple::array::umt_quick_sort_simple;
use rand::seq::SliceRandom;
use rand::rng;
use std::cmp::Ordering;

#[test]
fn benchmark_sort_string() {
    let size = 10_000;
    let mut rng = rng();
    let mut input: Vec<String> = (0..size).map(|i| i.to_string()).collect();
    input.shuffle(&mut rng);

    let start = Instant::now();
    // Specify generic types explicitly: T=String, F=function pointer
    let _result = umt_quick_sort_simple::<String, fn(&String, &String) -> Ordering>(
        &input,
        None,
        None,
        None
    );
    let duration = start.elapsed();

    println!("Sort String Benchmark: {:?}", duration);
}
