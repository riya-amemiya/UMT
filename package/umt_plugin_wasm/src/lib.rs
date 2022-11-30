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
pub fn deg_to_rad(degrees: f32) -> f32 {
    umt_deg_to_rad(degrees)
}

#[wasm_bindgen]
pub fn deviation_value(value: f64, average_value: f64, standard_deviation_value: f64) -> f64 {
    umt_deviation_value(value, average_value, standard_deviation_value)
}

#[wasm_bindgen]
pub fn factorize(n: i32) -> Vec<i32> {
    umt_factorize(n)
}