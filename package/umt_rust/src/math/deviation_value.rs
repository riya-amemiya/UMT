pub fn umt_deviation_value(value: f64, average_value: f64, standard_deviation_value: f64) -> f64 {
    (value - average_value) / standard_deviation_value * 10.0 + 50.0
}
