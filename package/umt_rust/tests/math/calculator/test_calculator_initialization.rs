use std::collections::HashMap;
use umt_rust::math::calculator::{umt_calculator_initialization, umt_calculator_initialization_fn};

#[test]
fn test_calculator_initialization_with_currency() {
    let mut rates = HashMap::new();
    rates.insert("$".to_string(), 100.0);
    let calc = umt_calculator_initialization(rates);
    assert_eq!(calc.calculate("$1"), "100");
}

#[test]
fn test_calculator_initialization_with_multiple_currencies() {
    let mut rates = HashMap::new();
    rates.insert("$".to_string(), 100.0);
    rates.insert("EUR".to_string(), 1.2);
    let calc = umt_calculator_initialization(rates);
    assert_eq!(calc.calculate("$1"), "100");
}

#[test]
fn test_calculator_initialization_fn_with_currency() {
    let mut rates = HashMap::new();
    rates.insert("EUR".to_string(), 1.2);
    let calc = umt_calculator_initialization_fn(rates);
    assert_eq!(calc("EUR5+10"), "16");
}

#[test]
fn test_calculator_initialization_regular_expression() {
    let rates = HashMap::new();
    let calc = umt_calculator_initialization(rates);
    assert_eq!(calc.calculate("1+2"), "3");
}

#[test]
fn test_calculator_initialization_complex_expression() {
    let rates = HashMap::new();
    let calc = umt_calculator_initialization(rates);
    assert_eq!(calc.calculate("(2+3)*4"), "20");
}
