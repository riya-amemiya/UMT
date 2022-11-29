mod math;

extern crate wasm_bindgen;
use math::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn average(numbers: Vec<f64>) -> f64 {
    umt_average_function(numbers)
}

#[wasm_bindgen]
pub fn get_decimal_length(value: f64) -> usize {
    umt_get_decimal_length(value)
}

#[wasm_bindgen]
pub fn factorial(x: i32) -> i32 {
    umt_factorial(x)
}

#[wasm_bindgen]
pub fn gcd(a: i64, b: i64) -> i64 {
    umt_gcd(a, b)
}

#[wasm_bindgen]
pub fn ncr(n: i32, r: i32) -> i32 {
    umt_ncr(n, r)
}

#[wasm_bindgen]
pub fn npr(n: i32, r: i32) -> i32 {
    umt_npr(n, r)
}

#[wasm_bindgen]
pub fn calculator(expression: String) -> String {
    umt_calculator(expression)
}
