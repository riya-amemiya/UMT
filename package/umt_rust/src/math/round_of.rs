pub fn umt_round_of(num: f64, precision: i32) -> f64 {
    return (num * 10f64.powi(precision)).round() / 10f64.powi(precision);
}
