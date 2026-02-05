use std::time::Instant;
use umt_rust::array::umt_chunk;

#[test]
fn benchmark_array_chunk() {
    let size = 1_000_000;
    let chunk_size = 100;
    let input: Vec<i32> = (0..size).collect();

    let start = Instant::now();
    let _result = umt_chunk(&input, chunk_size);
    let duration = start.elapsed();

    println!("Array Chunk Benchmark: {:?}", duration);
}
