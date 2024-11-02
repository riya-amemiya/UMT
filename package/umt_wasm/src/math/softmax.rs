use crate::round_of;

pub fn umt_softmax(x: Vec<f64>, decimal_place: i32) -> Vec<f64> {
    let max = x.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let exp: Vec<f64> = x.iter().map(|i| (i - max).exp()).collect();
    let sum = exp.iter().fold(0.0, |a, &b| a + b);
    exp.iter()
        .map(|i| round_of(i / sum, decimal_place))
        .collect()
}
