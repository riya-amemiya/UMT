
mod math;

extern crate wasm_bindgen;
use math::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn average(numbers: Vec<f64>) -> f64 {
    umt_average_function(numbers)
}

#[test]
fn test_average() {
    assert_eq!(average(vec![1.0, 2.0, 3.0]), 2.0);
}

#[wasm_bindgen]
pub fn get_decimal_length(value: f64) -> usize {
    umt_get_decimal_length(value)
}

#[wasm_bindgen]
pub fn multiplication(a: f64, b: f64) -> f64 {
    umt_multiplication(a, b)
}