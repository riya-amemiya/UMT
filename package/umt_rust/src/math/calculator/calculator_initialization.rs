use super::calc::umt_calculator;
use crate::object::Value;
use std::collections::HashMap;

/// Initializes a calculator function with exchange rates
///
/// # Arguments
///
/// * `exchange` - Exchange rates object
///
/// # Returns
///
/// Configured calculator closure
pub fn umt_calculator_initialization(exchange: HashMap<String, Value>) -> impl Fn(&str) -> String {
    move |x: &str| umt_calculator(x, Some(&exchange))
}
