//! Parity tests for the hexa_to_rgba module.

use umt_rust::color::{Rgba, umt_hexa_to_rgba};

fn rgba(r: f64, g: f64, b: f64, a: f64) -> Rgba {
    Rgba { r, g, b, a }
}

#[test]
fn test_hexa_to_rgba_parity_three_digit_code() {
    assert_eq!(
        umt_hexa_to_rgba("#abc").unwrap(),
        rgba(170.0, 187.0, 204.0, 1.0)
    );
}

#[test]
fn test_hexa_to_rgba_parity_six_digit_code() {
    assert_eq!(
        umt_hexa_to_rgba("#A1b2C3").unwrap(),
        rgba(161.0, 178.0, 195.0, 1.0)
    );
}

#[test]
fn test_hexa_to_rgba_parity_eight_digit_code() {
    assert_eq!(
        umt_hexa_to_rgba("#10203040").unwrap(),
        rgba(16.0, 32.0, 48.0, 0.25)
    );
    assert_eq!(
        umt_hexa_to_rgba("#aabbccdd").unwrap(),
        rgba(170.0, 187.0, 204.0, 0.87)
    );
    assert_eq!(
        umt_hexa_to_rgba("#000000fe").unwrap(),
        rgba(0.0, 0.0, 0.0, 1.0)
    );
}

#[test]
fn test_hexa_to_rgba_parity_non_ascii_digits_are_errors() {
    assert!(umt_hexa_to_rgba("#\u{0663}\u{0663}\u{0663}").is_err());
    assert!(umt_hexa_to_rgba("#\u{0663}\u{0663}\u{0663}\u{0663}").is_err());
    assert!(umt_hexa_to_rgba("#\u{FF11}\u{FF11}").is_err());
}

#[test]
fn test_hexa_to_rgba_parity_invalid_codes_are_errors() {
    assert!(umt_hexa_to_rgba("").is_err());
    assert!(umt_hexa_to_rgba("fff").is_err());
    assert!(umt_hexa_to_rgba("#ff").is_err());
    assert!(umt_hexa_to_rgba("#ffff").is_err());
    assert!(umt_hexa_to_rgba("#ggg").is_err());
    assert!(umt_hexa_to_rgba("#fff\n").is_err());
}
