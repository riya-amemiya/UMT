pub fn umt_value_swap(x: f64, y: f64) -> Vec<f64> {
    let mut x = x;
    let mut y = y;
    let temp = x;
    x = y;
    y = temp;
    vec![x, y]
}
