pub fn average_function(numbers: Vec<f64>) -> f64 {
    numbers.iter().sum::<f64>() / numbers.len() as f64
}