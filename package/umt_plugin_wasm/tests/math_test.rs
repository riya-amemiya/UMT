extern crate umt_plugin_wasm;
use umt_plugin_wasm::{average,get_decimal_length,factorial,gcd,ncr,npr};


#[test]
fn test_average() {
    assert_eq!(average(vec![1.0, 2.0, 3.0]), 2.0);
}

#[test]
fn test_get_decimal_length() {
    assert_eq!(get_decimal_length(1.0), 0);
    assert_eq!(get_decimal_length(1.1), 1);
    assert_eq!(get_decimal_length(1.11), 2);
    assert_eq!(get_decimal_length(1.111), 3);
}

#[test]
fn test_factorial() {
    assert_eq!(factorial(0), 1);
    assert_eq!(factorial(1), 1);
    assert_eq!(factorial(2), 2);
    assert_eq!(factorial(3), 6);
    assert_eq!(factorial(4), 24);
    assert_eq!(factorial(5), 120);
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(93, 169), 1);
    assert_eq!(gcd(81, 27), 27);
}

#[test]
fn test_npr() {
    assert_eq!(npr(5, 2), 20);
    assert_eq!(npr(5, 3), 60);
    assert_eq!(npr(5, 4), 120);
    assert_eq!(npr(5, 5), 120);
}

#[test]
fn test_ncr() {
    assert_eq!(ncr(5, 2), 10);
    assert_eq!(ncr(5, 3), 10);
    assert_eq!(ncr(5, 4), 5);
    assert_eq!(ncr(5, 5), 1);
}
