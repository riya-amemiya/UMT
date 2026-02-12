extern crate wasm_bindgen;

use umt_rust::{array::*, math::*};
use wasm_bindgen::prelude::*;

// array functions

#[wasm_bindgen]
pub fn range(min: i32, max: i32) -> Vec<i32> {
    umt_range(min, Some(max), None)
}

// math functions

#[wasm_bindgen]
pub fn average(numbers: Vec<f64>) -> f64 {
    umt_average(numbers)
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
pub fn factorial(x: i32) -> i32 {
    umt_factorial(x)
}

#[wasm_bindgen]
pub fn factorize(n: i32) -> Vec<i32> {
    umt_factorize(n)
}

#[wasm_bindgen]
pub fn gcd(a: i32, b: i32) -> i32 {
    umt_gcd(a, b)
}

#[wasm_bindgen]
pub fn get_decimal_length(value: f64) -> usize {
    umt_get_decimal_length(value)
}

#[wasm_bindgen]
pub fn lcm(a: i32, b: i32) -> i32 {
    umt_lcm(a, b)
}

#[wasm_bindgen]
pub fn linear_congruential_generator(
    seed: i32,
    max: Option<i32>,
    multiplier: Option<i32>,
    increment: Option<i32>,
) -> i32 {
    umt_linear_congruential_generator(seed, max, multiplier, increment)
}

#[wasm_bindgen]
pub fn math_separator(x: i32) -> Vec<i32> {
    umt_math_separator(x)
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
pub fn round_of(value: f64, decimal_place: i32) -> f64 {
    umt_round_of(value, decimal_place)
}

#[wasm_bindgen]
pub fn softmax(x: Vec<f64>, decimal_place: i32) -> Vec<f64> {
    umt_softmax(x, decimal_place)
}

#[wasm_bindgen]
pub fn value_swap(x: f64, y: f64) -> Vec<f64> {
    umt_value_swap(x, y)
}
