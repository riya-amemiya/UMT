use std::collections::HashMap;
use umt_rust::math::calculator::umt_convert_currency;

#[test]
fn test_convert_yen_to_base() {
    let mut rates = HashMap::new();
    rates.insert("¥".to_string(), 0.01);
    assert_eq!(umt_convert_currency("¥100", Some(&rates)), "1");
}

#[test]
fn test_convert_dollar_to_base() {
    let mut rates = HashMap::new();
    rates.insert("$".to_string(), 1.2);
    assert_eq!(umt_convert_currency("$50", Some(&rates)), "60");
}

#[test]
fn test_convert_euro_to_base() {
    let mut rates = HashMap::new();
    rates.insert("€".to_string(), 1.1);
    assert_eq!(umt_convert_currency("€200", Some(&rates)), "220");
}

#[test]
fn test_no_conversion_rates() {
    assert_eq!(umt_convert_currency("$100", None), "$100");
}

#[test]
fn test_unknown_currency() {
    let mut rates = HashMap::new();
    rates.insert("$".to_string(), 1.0);
    assert_eq!(umt_convert_currency("€100", Some(&rates)), "€100");
}

#[test]
fn test_conversion_with_100_rate() {
    let mut rates = HashMap::new();
    rates.insert("$".to_string(), 100.0);
    assert_eq!(umt_convert_currency("$1", Some(&rates)), "100");
}

#[test]
fn test_conversion_with_fractional_rate() {
    let mut rates = HashMap::new();
    rates.insert("£".to_string(), 0.5);
    assert_eq!(umt_convert_currency("£10", Some(&rates)), "5");
}

#[test]
fn test_conversion_preserves_unmatched_input() {
    let mut rates = HashMap::new();
    rates.insert("$".to_string(), 1.0);
    assert_eq!(umt_convert_currency("100", Some(&rates)), "100");
}
