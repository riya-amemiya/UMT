
mod math;

extern crate wasm_bindgen;
use math::average_function;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn average(numbers: Vec<f64>) -> f64 {
    average_function(numbers)
}