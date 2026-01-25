//! Tests for the rgba_to_cmyk module.

use umt_rust::color::{umt_rgba_to_cmyk, Cmyk, RgbaInput};

#[test]
fn test_rgba_to_cmyk_invalid_values() {
    assert!(umt_rgba_to_cmyk(RgbaInput { r: -1.0, g: 0.0, b: 0.0, a: Some(1.0) }).is_err());
    assert!(umt_rgba_to_cmyk(RgbaInput { r: 256.0, g: 0.0, b: 0.0, a: Some(1.0) }).is_err());
    assert!(umt_rgba_to_cmyk(RgbaInput { r: 0.0, g: -1.0, b: 0.0, a: Some(1.0) }).is_err());
    assert!(umt_rgba_to_cmyk(RgbaInput { r: 0.0, g: 256.0, b: 0.0, a: Some(1.0) }).is_err());
    assert!(umt_rgba_to_cmyk(RgbaInput { r: 0.0, g: 0.0, b: -1.0, a: Some(1.0) }).is_err());
    assert!(umt_rgba_to_cmyk(RgbaInput { r: 0.0, g: 0.0, b: 256.0, a: Some(1.0) }).is_err());
    assert!(umt_rgba_to_cmyk(RgbaInput { r: 0.0, g: 0.0, b: 0.0, a: Some(-0.1) }).is_err());
    assert!(umt_rgba_to_cmyk(RgbaInput { r: 0.0, g: 0.0, b: 0.0, a: Some(1.1) }).is_err());
}

#[test]
fn test_rgba_to_cmyk_error_message() {
    let err = umt_rgba_to_cmyk(RgbaInput { r: -1.0, g: 0.0, b: 0.0, a: Some(1.0) }).unwrap_err();
    assert_eq!(err.message, "Invalid rgba value");
}

#[test]
fn test_rgba_to_cmyk_conversions() {
    // White
    let result = umt_rgba_to_cmyk(RgbaInput { r: 255.0, g: 255.0, b: 255.0, a: Some(1.0) }).unwrap();
    assert_eq!(result, Cmyk { c: 0.0, m: 0.0, y: 0.0, k: 0.0, a: 1.0 });

    // Black
    let result = umt_rgba_to_cmyk(RgbaInput { r: 0.0, g: 0.0, b: 0.0, a: Some(1.0) }).unwrap();
    assert_eq!(result, Cmyk { c: 0.0, m: 0.0, y: 0.0, k: 100.0, a: 1.0 });

    // Red with alpha 0.5
    let result = umt_rgba_to_cmyk(RgbaInput { r: 255.0, g: 0.0, b: 0.0, a: Some(0.5) }).unwrap();
    assert_eq!(result, Cmyk { c: 0.0, m: 100.0, y: 100.0, k: 0.0, a: 0.5 });

    // Green with alpha 0.7
    let result = umt_rgba_to_cmyk(RgbaInput { r: 0.0, g: 255.0, b: 0.0, a: Some(0.7) }).unwrap();
    assert_eq!(result, Cmyk { c: 100.0, m: 0.0, y: 100.0, k: 0.0, a: 0.7 });

    // Blue with alpha 0.3
    let result = umt_rgba_to_cmyk(RgbaInput { r: 0.0, g: 0.0, b: 255.0, a: Some(0.3) }).unwrap();
    assert_eq!(result, Cmyk { c: 100.0, m: 100.0, y: 0.0, k: 0.0, a: 0.3 });
}

#[test]
fn test_rgba_to_cmyk_default_alpha() {
    // White without alpha (defaults to 1.0)
    let result = umt_rgba_to_cmyk(RgbaInput { r: 255.0, g: 255.0, b: 255.0, a: None }).unwrap();
    assert_eq!(result, Cmyk { c: 0.0, m: 0.0, y: 0.0, k: 0.0, a: 1.0 });
}
