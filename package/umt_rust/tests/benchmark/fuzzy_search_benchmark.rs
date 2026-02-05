use std::time::Instant;
use umt_rust::string::umt_fuzzy_search;

#[test]
fn benchmark_fuzzy_search() {
    let iterations = 10_000;
    let query = "apple";
    let text = "apricot application apple pie";
    let items = vec![text]; // Search in list

    let start = Instant::now();
    for _ in 0..iterations {
        let _ = umt_fuzzy_search(query, &items, 0.5);
    }
    let duration = start.elapsed();

    println!("Fuzzy Search Benchmark: {:?}", duration);
}
