use umt_rust::math::calculator::{umt_calculator, umt_calculator_core};
use umt_rust::object::Value;
use umt_rust::obj;
use std::collections::HashMap;

#[test]
fn test_core_basic_operations() {
    assert_eq!(umt_calculator_core("1+1", None), "2");
    assert_eq!(umt_calculator_core("10+20", None), "30");
    assert_eq!(umt_calculator_core("0+0", None), "0");

    assert_eq!(umt_calculator_core("5-3", None), "2");
    assert_eq!(umt_calculator_core("10-20", None), "-10");

    assert_eq!(umt_calculator_core("3*4", None), "12");
    assert_eq!(umt_calculator_core("5*0", None), "0");

    assert_eq!(umt_calculator_core("8/2", None), "4");
    assert_eq!(umt_calculator_core("1/1", None), "1");
    // 10/3 -> 3.3333...
    let result = umt_calculator_core("10/3", None);
    assert!(result.starts_with("3.3333"));

    assert_eq!(umt_calculator_core("2^3", None), "8");
    assert_eq!(umt_calculator_core("5^2", None), "25");

    // Test mixed precedence
    assert_eq!(umt_calculator_core("2*3+4", None), "10");
    assert_eq!(umt_calculator_core("4+2*3", None), "10");
    assert_eq!(umt_calculator_core("10/2*5", None), "25"); // Left associative
}

#[test]
fn test_core_signs() {
    assert_eq!(umt_calculator_core("5--3", None), "8");
    assert_eq!(umt_calculator_core("5++3", None), "8");
    assert_eq!(umt_calculator_core("5+-3", None), "2");
}

#[test]
fn test_core_parentheses() {
    assert_eq!(umt_calculator_core("(2+3)", None), "5");
    assert_eq!(umt_calculator_core("(10-5)", None), "5");
    assert_eq!(umt_calculator_core("(3*4)", None), "12");
    assert_eq!(umt_calculator_core("(2+3)*4", None), "20");
}

#[test]
fn test_core_currency() {
    let exchange = obj!("$" => 100);
    if let Value::Object(map) = exchange {
        assert_eq!(umt_calculator_core("$10*2", Some(&map)), "2000");
    }

    let exchange_euro = obj!("$" => 100, "E" => 110);
    if let Value::Object(map) = exchange_euro {
        // Just verify it processes
        let res = umt_calculator_core("$10+E5", Some(&map));
        // 10*100 + 5*110 = 1000 + 550 = 1550
        assert_eq!(res, "1550");
    }
}

#[test]
fn test_core_decimals() {
    assert_eq!(umt_calculator_core("2.5+1.5", None), "4");
    assert_eq!(umt_calculator_core("0.1+0.2", None), "0.3"); // Floating point check
}

#[test]
fn test_calculator_function() {
    // Should behave like core for expressions
    assert_eq!(umt_calculator("1+1", None), "2");
    assert_eq!(umt_calculator("1+1+1", None), "3");

    // Equations
    // 2x=6 -> x=3
    assert_eq!(umt_calculator("2x=6", None), "3");
    assert_eq!(umt_calculator("x+1=2", None), "1");
    assert_eq!(umt_calculator("3x+2=8", None), "2"); // 3x=6 -> x=2

    // Complex equations
    // 2x = (1+1)+(1+1)+(1+1) -> 2x = 6 -> 3
    assert_eq!(umt_calculator("2x=(1+1)+(1+1)+(1+1)", None), "3");

    // Multiple constants on variable side
    assert_eq!(umt_calculator("2x+1+2=15", None), "6");
}
