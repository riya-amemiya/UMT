pub fn umt_get_decimal_length(value: f64) -> usize {
    let value_string = value.to_string();
    let value_string_split: Vec<&str> = value_string.split(".").collect();
    if value_string_split.len() == 1 {
        return 0;
    }
    value_string_split[1].len()
}
