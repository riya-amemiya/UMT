use std::time::Instant;
use umt_rust::array::umt_get_arrays_common;

#[test]
fn benchmark_get_arrays_common() {
    let size = 10_000;
    let arr1: Vec<i32> = (0..size).collect();
    let arr2: Vec<i32> = (size/2..size + size/2).collect();

    let start = Instant::now();
    let _result = umt_get_arrays_common(&[&arr1, &arr2]);
    let duration = start.elapsed();

    println!("Get Arrays Common Benchmark: {:?}", duration);
}
