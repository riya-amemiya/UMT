pub fn umt_get_decimal_length(value: f64) -> usize {
    let valuse_string = value.to_string();
    let x = valuse_string.split('.').last().unwrap_or("");
    if x.len() > 0 {
        return x.len();
    }
    return 0;
}