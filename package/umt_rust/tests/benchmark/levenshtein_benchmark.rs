use std::time::Instant;
use umt_rust::string::umt_levenshtein_distance;

#[test]
fn benchmark_levenshtein() {
    let iterations = 10_000;
    let s1 = "kitten";
    let s2 = "sitting";

    let start = Instant::now();
    for _ in 0..iterations {
        let _ = umt_levenshtein_distance(s1, s2);
    }
    let duration = start.elapsed();

    println!("Levenshtein Benchmark: {:?}", duration);
}
