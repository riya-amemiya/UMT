//! Tests for the hexa_to_rgba module.

use umt_rust::color::{Rgba, umt_hexa_to_rgba};

#[test]
fn test_hexa_to_rgba_6_digit() {
    assert_eq!(
        umt_hexa_to_rgba("#FF0000").unwrap(),
        Rgba {
            r: 255.0,
            g: 0.0,
            b: 0.0,
            a: 1.0
        }
    );
    assert_eq!(
        umt_hexa_to_rgba("#00FF00").unwrap(),
        Rgba {
            r: 0.0,
            g: 255.0,
            b: 0.0,
            a: 1.0
        }
    );
    assert_eq!(
        umt_hexa_to_rgba("#0000FF").unwrap(),
        Rgba {
            r: 0.0,
            g: 0.0,
            b: 255.0,
            a: 1.0
        }
    );
    assert_eq!(
        umt_hexa_to_rgba("#FFFFFF").unwrap(),
        Rgba {
            r: 255.0,
            g: 255.0,
            b: 255.0,
            a: 1.0
        }
    );
    assert_eq!(
        umt_hexa_to_rgba("#000000").unwrap(),
        Rgba {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0
        }
    );
    assert_eq!(
        umt_hexa_to_rgba("#FFA500").unwrap(),
        Rgba {
            r: 255.0,
            g: 165.0,
            b: 0.0,
            a: 1.0
        }
    );
}

#[test]
fn test_hexa_to_rgba_8_digit_with_alpha() {
    assert_eq!(
        umt_hexa_to_rgba("#FFA50099").unwrap(),
        Rgba {
            r: 255.0,
            g: 165.0,
            b: 0.0,
            a: 0.6
        }
    );
}

#[test]
fn test_hexa_to_rgba_3_digit() {
    assert_eq!(
        umt_hexa_to_rgba("#F00").unwrap(),
        Rgba {
            r: 255.0,
            g: 0.0,
            b: 0.0,
            a: 1.0
        }
    );
    assert_eq!(
        umt_hexa_to_rgba("#0F0").unwrap(),
        Rgba {
            r: 0.0,
            g: 255.0,
            b: 0.0,
            a: 1.0
        }
    );
    assert_eq!(
        umt_hexa_to_rgba("#00F").unwrap(),
        Rgba {
            r: 0.0,
            g: 0.0,
            b: 255.0,
            a: 1.0
        }
    );
    assert_eq!(
        umt_hexa_to_rgba("#FFF").unwrap(),
        Rgba {
            r: 255.0,
            g: 255.0,
            b: 255.0,
            a: 1.0
        }
    );
    assert_eq!(
        umt_hexa_to_rgba("#000").unwrap(),
        Rgba {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0
        }
    );
}

#[test]
fn test_hexa_to_rgba_8_digit() {
    assert_eq!(
        umt_hexa_to_rgba("#FF0000FF").unwrap(),
        Rgba {
            r: 255.0,
            g: 0.0,
            b: 0.0,
            a: 1.0
        }
    );
    assert_eq!(
        umt_hexa_to_rgba("#FF000080").unwrap(),
        Rgba {
            r: 255.0,
            g: 0.0,
            b: 0.0,
            a: 0.5
        }
    );
    assert_eq!(
        umt_hexa_to_rgba("#FF000000").unwrap(),
        Rgba {
            r: 255.0,
            g: 0.0,
            b: 0.0,
            a: 0.0
        }
    );
}

#[test]
fn test_hexa_to_rgba_invalid_hex_code() {
    assert!(umt_hexa_to_rgba("#12345").is_err());
    assert!(umt_hexa_to_rgba("#1234567").is_err());
    assert!(umt_hexa_to_rgba("123456").is_err());
    assert!(umt_hexa_to_rgba("").is_err());
}

#[test]
fn test_hexa_to_rgba_error_message() {
    let err = umt_hexa_to_rgba("#12345").unwrap_err();
    assert_eq!(err.message, "Invalid hex code");
}

use umt_rust::color::*;

#[test]
fn test_hexa_to_rgba_invalid() {
    assert!(umt_hexa_to_rgba("#12345").is_err());
    assert!(umt_hexa_to_rgba("#1234567").is_err());
    assert!(umt_hexa_to_rgba("123456").is_err());
    assert!(umt_hexa_to_rgba("").is_err());
}
