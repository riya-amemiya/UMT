use std::time::Instant;
use umt_rust::math::calculator::umt_calculator;
use umt_rust::object::Value;
use std::collections::HashMap;

#[test]
fn benchmark_calculator_currency() {
    let iterations = 10_000;
    let mut rates = HashMap::new();
    rates.insert("USD".to_string(), Value::Float(1.0));
    rates.insert("EUR".to_string(), Value::Float(0.85));
    let exchange = Some(&rates);

    let start = Instant::now();
    for _ in 0..iterations {
        let _ = umt_calculator("USD100 + EUR50", exchange);
    }
    let duration = start.elapsed();

    println!("Calculator Currency Benchmark: {:?}", duration);
}
