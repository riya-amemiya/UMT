
pub fn umt_multiplication(a: f64, b: f64) -> f64 {
    let decimal_length:u32 = crate::get_decimal_length(a) as u32 + crate::get_decimal_length(b) as u32;
    let ten:u32 = 10;
    let n: f64= ten.pow(decimal_length) as f64;
    let x = a.to_string().replace(".", "").parse::<f64>().unwrap();
    let y = b.to_string().replace(".", "").parse::<f64>().unwrap();
    (x * y) / n
}